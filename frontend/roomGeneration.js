function createRoom() {
    var roomId = makeid();
    var playerName = document.getElementById("playerName").value;
    if (playerName.length == 0) {
        alert("You must choose a player name");
        return;
    }
    goToRoom(roomId, playerName, true);
}

function joinRoom() {
    var joinStatus = window.getComputedStyle(document.getElementById("joinRoomForm")).display;
    if (joinStatus === "none") {
        document.getElementById("joinRoomForm").style.display = "inline-block";
        document.getElementById("createRoomForm").style.display = "none";
        return;
    }

    var playerName = document.getElementById("playerName").value;
    if (playerName.length == 0) {
        alert("You must choose a player name");
        return;
    }
    var validCode = validateRoomCode();
    if (!validCode) {
        alert ("Room code must be 5 characters and alphanumeric (no special characters)")
        return;
    }
    goToRoom(document.getElementById("roomCode").value.toUpperCase(), playerName, false);
}

function validateRoomCode() {
    var roomCode = document.getElementById("roomCode").value.toUpperCase();
    var playerName = document.getElementById("playerName").value;
    var re = new RegExp("^[A-Z0-9]+$");
    valid = true;
    if (roomCode.length != 5) {
        valid = false;
    } else if (!re.test(roomCode)) {
        valid = false;
    }

    return valid;
    
}

function goToRoom(roomId, playerName, newRoom) {
    sessionStorage.setItem("playerName", playerName);
    sessionStorage.setItem("newRoom", newRoom);
    window.location = "canvas.html?room=" + roomId;
}

function makeid() {
    var result           = '';
    var characters       = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
    var charactersLength = characters.length;
    for ( var i = 0; i < 5; i++ ) {
       result += characters.charAt(Math.floor(Math.random() * charactersLength));
    }
    return result;
 }