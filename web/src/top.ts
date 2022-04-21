const socket = new WebSocket(`ws://${document.location.host}${document.location.pathname}`);

document.addEventListener('DOMContentLoaded', connect);

/**
 * Connect to the server.
 */
function connect() {
  socket.onopen = onOpen;
  socket.onmessage = onMessage;
  socket.onerror = onError;
  socket.onclose = onClose;
}

/**
 * @param {HTMLInputElement} input The input field that was changed.
 * @param {string} value The new value.
 */
function update(input: HTMLInputElement, value: string = input.value) {
  input.classList.remove('is-success');
  input.classList.remove('is-danger');
  input.classList.add('is-loading');
  const message = JSON.stringify({
    update: {
      id: input.id,
      value: value,
    },
  });
  socket.send(message);
  console.log(`sent: ${message}`);
}

/**
 * @param {HTMLButtonElement} button The button that was pressed.
 */
function press(button: HTMLButtonElement) {
  const message = JSON.stringify({
    press: {
      id: button.id,
    },
  });
  socket.send(message);
  console.log(`sent: ${message}`);
}

/**
 * @param {Event} ev The event.
 */
function onOpen(ev: Event) {
  console.log('connected');
}

/**
 * @param {MessageEvent} ev The event.
 */
function onMessage(ev: MessageEvent) {
  console.log(`received: ${ev.data}`);
  const data = JSON.parse(ev.data);
  data.forEach((change: any) => {
    if (change.replaceContent != null) {
      const element = document.getElementById(change.replaceContent.id);
      if (element != null) {
        element.innerHTML = change.replaceContent.html;
      }
    } else if (change.appendContent != null) {
      const template = document.createElement('template');
      template.innerHTML = change.appendContent.html;
      const element = document.getElementById(change.appendContent.id);
      element?.appendChild(template.content);
    } else if (change.remove != null) {
      const element = document.getElementById(change.remove.id);
      element?.parentElement?.removeChild(element);
    } else if (change.valid != null) {
      const id = change.valid.id;
      const input = document.getElementById(id);
      input?.classList.remove('is-loading');
      input?.classList.add('is-success');
    } else if (change.invalid != null) {
      const id = change.invalid.id;
      const input = document.getElementById(id);
      input?.classList.remove('is-loading');
      input?.classList.add('is-danger');
    }
  });
}

/**
 * @param {Event} ev The event.
 */
function onError(ev: Event) {
  console.log(`connection error`);
  alert('Failed to connect to the server');
}

/**
 * @param {CloseEvent} ev The event.
 */
function onClose(ev: CloseEvent) {
  console.log(`lost connection: ${ev.reason}`);
  alert('Lost connection');
}
