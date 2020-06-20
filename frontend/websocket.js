var connected = false;
var ws;

/**
 * attemptConnection
 * Attempts to connect to the websocket.
 */
function attemptConnection() {
    ws = new WebSocket("wss://ws.squaresplosion.com:9999");
    ws.onmessage = (e) => {
        var boardState = JSON.parse(e.data);
        console.log(boardState);
        updatePlayerLocations(boardState.player_state);
        
    }
    ws.onopen = () => {
        var url = new URL(window.location.href);
        var room = url.searchParams.get("room");
        document.getElementById("roomCode").innerText = room;
        var roomOwner = sessionStorage.getItem("newRoom") === "true";
        var player = sessionStorage.getItem("playerName");
        if (player === null) {
            while (player === "" || player === null) {
                var player = prompt("Enter your player name (this name will be saved for future games):");
            }
            sessionStorage.setItem("playerName", player);
        }
        connected = true;
        ws.send(JSON.stringify({id: character.id, player_name: player, room_name: room, x: character.x, y: character.y, new_room: roomOwner}));
        sessionStorage.removeItem("newRoom");
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
    ws.send(JSON.stringify({x: xPos, y: yPos}));
}
