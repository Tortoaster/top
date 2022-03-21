const socket = new WebSocket(`ws://${document.location.host}${document.location.pathname}`);

document.addEventListener('DOMContentLoaded', connect);

/**
 * Connect to the server.
 */
function connect(ev: Event) {
  socket.onopen = onOpen;
  socket.onmessage = onMessage;
  socket.onerror = onError;
  socket.onclose = onClose;
}

/**
 * @param {HTMLInputElement} input The input field that was changed.
 */
function update(input: HTMLInputElement) {
  if(input.attributes.getNamedItem('synced') != null)
    input.attributes.removeNamedItem('synced');
  if(input.attributes.getNamedItem('failed') != null)
    input.attributes.removeNamedItem('failed');
  input.attributes.setNamedItem(document.createAttribute('syncing'));
  const message = JSON.stringify({
    update: {
      id: input.id,
      value: input.value,
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
    const element = document.getElementById(data.replace.id) as HTMLDivElement;
    element.innerHTML = data.replace.component;
  } else if(data.append != null) {
    const element = document.getElementById(data.append.id) as HTMLDivElement;
    element.innerHTML += data.append.component;
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
  console.error(`error: ${ev}`);
  alert(`Error: ${ev}`);
}

/**
 * @param {CloseEvent} ev The event.
 */
function onClose(ev: CloseEvent) {
  console.log('disconnected');
  alert('Disconnected');
}
