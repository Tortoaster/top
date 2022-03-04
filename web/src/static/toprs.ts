const CONTENT = 'toprs-content';

const socket = new WebSocket(`ws://${document.location.host}/ws`);

document.addEventListener('DOMContentLoaded', connect);

/**
 * Connect to the TopRs server.
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
  const data = JSON.parse(ev.data);
  console.log(`received: ${data}`);
  const content = document.getElementById(CONTENT) as HTMLDivElement;
  if(data.newContent !== null) {
    content.innerHTML = data.newContent.content;
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
