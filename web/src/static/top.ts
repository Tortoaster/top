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
  if(input.classList.contains('is-success'))
    input.classList.remove('is-success');
  if(input.classList.contains('is-danger'))
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
  if(data.replace != null) {
    const template = document.createElement('template');
    template.innerHTML = data.replace.html;

    const element = document.getElementById(data.replace.id);
    element?.replaceWith(template.content);
  } else if(data.insert != null) {
    const template = document.createElement('template');
    template.innerHTML = data.insert.html;

    const element = document.getElementById(data.insert.id);
    element?.appendChild(template.content);
  } else if(data.remove != null) {
    const element = document.getElementById(data.remove.id);
    element?.parentElement?.removeChild(element);
  } else if(data.valid != null) {
    const id = data.valid.id;
    const input = document.getElementById(id);
    input?.classList.remove('is-loading');
    input?.classList.add('is-success');
  } else if(data.invalid != null) {
    const id = data.invalid.id;
    const input = document.getElementById(id);
    input?.classList.remove('is-loading');
    input?.classList.add('is-danger');
  }
}

/**
 * @param {Event} ev The event.
 */
function onError(ev: Event) {
  console.log(`connection error`);
  alert('Failed to connect to the server')
}

/**
 * @param {CloseEvent} ev The event.
 */
function onClose(ev: CloseEvent) {
  console.log(`lost connection: ${ev.reason}`);
  alert('Lost connection')
}
