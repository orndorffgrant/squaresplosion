function createRoom() {
    var createStatus = window.getComputedStyle(document.getElementById("createRoomForm")).display;
    if (createStatus === "none") {
        document.getElementById("joinRoomForm").style.display = "none";
        document.getElementById("createRoomForm").style.display = "inline-block";
        return;
    }

    var roomCode = document.getElementById("newRoomCode").value.toUpperCase();
    var playerName = document.getElementById("playerName").value;
    if (playerName.length == 0) {
        alert("You must choose a player name");
        return;
    }
    var validCode = validateRoomCode(roomCode);
    if (!validCode) {
        alert("Room name cannot have special characters");
        return;
    }
    goToRoom(roomCode, playerName, true);
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
    var roomCode = document.getElementById("roomCode").value.toUpperCase();
    var validCode = validateRoomCode(roomCode);
    if (!validCode) {
        alert ("Room name cannot have special characters")
        return;
    }
    goToRoom(document.getElementById("roomCode").value.toUpperCase(), playerName, false);
}

function validateRoomCode(roomCode) {
    var re = new RegExp("^[A-Z0-9]+$");
    valid = true;
    if (!re.test(roomCode)) {
        valid = false;
    }

    return valid;
    
}

function goToRoom(roomId, playerName, newRoom) {
    sessionStorage.setItem("playerName", playerName);
    sessionStorage.setItem("newRoom", newRoom);
    window.location.href = "canvas.html?room=" + roomId;
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