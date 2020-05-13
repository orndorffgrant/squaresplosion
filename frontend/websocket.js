var connected = false;
var ws;
attemptConnection();

/**
 * attemptConnection
 * Attempts to connect to the websocket.
 */
function attemptConnection() {
    ws = new WebSocket("ws://localhost:9999");
    ws.onmessage = (e) => {
        var player = JSON.parse(e.data);
        var playerExists = false;
        for (var i = 0; i < otherPlayers.length; i++) {
            if (otherPlayers[i].id == player.id) {
                otherPlayers[i].x = player.x;
                otherPlayers[i].y = player.y;
                playerExists = true;
                break;
            }
        }
        if (!playerExists) {
            createOtherPlayer(player.id, player.x, player.y);
        }
    }
    ws.onopen = () => {
        connected = true;
        ws.send(JSON.stringify({id: character.id, x: character.x, y: character.y}));
    }
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
