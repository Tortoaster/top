let socket!: WebSocket;

document.addEventListener('DOMContentLoaded', connect);

/**
 * Connect to the server.
 */
function connect() {
  socket = new WebSocket(`ws://${document.location.host}${document.location.pathname}`);
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
  if(input.attributes.getNamedItem('synced') != null)
    input.attributes.removeNamedItem('synced');
  if(input.attributes.getNamedItem('failed') != null)
    input.attributes.removeNamedItem('failed');
  input.attributes.setNamedItem(document.createAttribute('syncing'));
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
  if(data.insert != null) {
    const element = document.getElementById(data.insert.id) as HTMLDivElement;
    element.innerHTML = data.insert.html;
  } else if(data.replace != null) {
    const element = document.getElementById(data.replace.id) as HTMLDivElement;
    const template = document.createElement('template');
    template.innerHTML = data.replace.html;
    element.replaceWith(template.content);
  } else if(data.append != null) {
    const element = document.getElementById(data.append.id) as HTMLDivElement;
    const template = document.createElement('template');
    template.innerHTML = data.append.html;
    element.appendChild(template.content);
  } else if(data.remove != null) {
    const element = document.getElementById(data.remove.id) as HTMLDivElement;
    element.parentElement?.removeChild(element);
  } else if(data.valid != null) {
    const id = data.valid.id;
    const input = document.getElementById(id) as HTMLElement;
    input.attributes.removeNamedItem('syncing');
    input.attributes.setNamedItem(document.createAttribute('synced'));
  } else if(data.invalid != null) {
    const id = data.invalid.id;
    const input = document.getElementById(id) as HTMLElement;
    input.attributes.removeNamedItem('syncing');
    input.attributes.setNamedItem(document.createAttribute('failed'));
  }
}

/**
 * @param {Event} ev The event.
 */
function onError(ev: Event) {
  setTimeout(connect, 5000);
}

/**
 * @param {CloseEvent} ev The event.
 */
function onClose(ev: CloseEvent) {
  console.log(`lost connection: ${ev.reason}`);
  setTimeout(connect, 1000);
}
