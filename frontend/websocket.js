var connected = false;
var ws;
attemptConnection();

/**
 * attemptConnection
 * Attempts to connect to the websocket.
 */
function attemptConnection() {
    ws = new WebSocket("ws://localhost:9999");
    ws.onmessage = (e) => console.log(e);
    waitForSocketConnection(ws, function() {
        connected = true;
    })
}


/**
 * sendLocation
 * sends the location of the character to the web socket
 * @param string charId : the character id
 * @param int xPos : the character x position
 * @param int yPos : the character y position
 */
function sendLocation(charId, xPos, yPos){
    if (!connected) {
        console.error("Not connected");
        return;
    }
    ws.send(JSON.stringify({id: charId, x: xPos, y: yPos}));
}

/**
 * waitForSocketConnection
 * @param WebSocket socket : the websocket to wait for
 * @param function callback : what function to call on success
 */
function waitForSocketConnection(socket, callback){
    setTimeout(
        function () {
            if (socket.readyState === 1) {
                console.log("Connection is made")
                if (callback != null){
                    callback();
                }
            } else {
                console.log("wait for connection...")
                waitForSocketConnection(socket, callback);
            }

        }, 5); // wait 5 milisecond for the connection...
}
