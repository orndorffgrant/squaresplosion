var character;
var bots = [];
var gameMat;

/**
 * createGameMat
 *   Creates the game mat based on the size of the canvas.
 */
function createGameMat() {
    gameMat = {
        canvas: document.getElementById("grid"),
        start: function() {
            this.canvas.width = this.canvas.getAttribute("width");
            this.canvas.height = this.canvas.getAttribute("height");;
            this.context = this.canvas.getContext("2d");
            document.body.insertBefore(this.canvas, document.body.childNodes[0]);
            this.interval = setInterval(updateGameArea, 20);
        },
        clear: function() {
            this.context.clearRect(0, 0, this.canvas.width, this.canvas.height);
        }
    }   
}

/**
 * getRandomColor
 *   Gets a random color from a list of approved colors
 *   return string (the color)
 */
function getRandomColor() {
    colors = ["#4287f5", "#4287f5", "#f542a4", "#f54242","#f59942", "#93f542", "#42f5e3"]
    return colors[Math.floor(Math.random() * 7)]
  }

/**
 * startGame()
 *   Start the game! Calls function to create active player and game mat
 */
function startGame() {
    createGameMat();
    var randomX = Math.floor(Math.random() * ((gameMat.canvas.width / 25))) * 25;
    var randomY = Math.floor(Math.random() * ((gameMat.canvas.height / 25))) * 25;
    var randomColor = getRandomColor();
    document.getElementById("grid").style.borderColor = randomColor;
    character = new component(25, 25, randomColor, randomX, randomY, "player");
    addListener();
    gameMat.start();
}

/**
 * addBot() 
 *   Adds a bot that will move randomly every 1/4 a second
 */
function addBot() {
    botXY = [];
    while (botXY.length == 0) {
        botXY = getBotValidPosition();
    }
    bot = new component(25, 25, "#000000", botXY[0], botXY[1], "bot");
    bots.push(bot);
    if (bots.length == 1) {
        setInterval(function() {moveBots()}, 50);
    }
}

/**
 * getBotValidPosition()
 *   Makes sure that the bot we are creating is not on top of another bot or player
 *   return array of [X position, Y position]
 */
function getBotValidPosition() {
    validPosition = true;
    var randomX = Math.floor(Math.random() * ((gameMat.canvas.width / 25))) * 25;
    var randomY = Math.floor(Math.random() * ((gameMat.canvas.height / 25))) * 25;
    
    for (var i = 0; i < bots.length; i++) {
        if (randomX == bots[i].x && randomY == bots[i].y) {
            validPosition = false;
            break;
        }
    }

    if (randomX == character.x && randomY == character.y) {
        validPosition = false;
    }

    if (validPosition) {
        return [randomX, randomY];
    }
    return [];
}

/**
 * moveBots()
 *   Generates a random number between 0 and 4. Each number means a different move. 
 */
function moveBots() {
    var moved;
    for (var i = 0; i < bots.length; i++) {
        moved = false;
        const cont = Math.floor(Math.random() * 2) === 0;
        if (cont) {
            move = bots[i].lastmove;
        } else {
            move = Math.floor(Math.random() * 5);
        }
        switch(move) {
            case 0:
                if (bots[i].y - 25 >= 0) {
                    bots[i].y -= 25;
                    bots[i].lastmove = move;
                    moved = true;
                }
                break;
            case 1:
                if (bots[i].y + 25 <= gameMat.canvas.height - 25) {
                    bots[i].y += 25;
                    bots[i].lastmove = move;
                    moved = true;
                }
                break;
            case 2:
                if (bots[i].x - 25 >= 0) {
                    bots[i].x -= 25;
                    bots[i].lastmove = move;
                    moved = true;
                }
                break;
            case 3:
                if (bots[i].x + 25 <= gameMat.canvas.width - 25) {
                    bots[i].x += 25;
                    bots[i].lastmove = move;
                    moved = true;
                }
                break;           
            default:
                break;
        }
        if (moved) {
            sendLocation(bots[i].id, bots[i].x, bots[i].y);
            bots[i].lastMoveTime = (new Date()).getTime();
        }
    }
}

/**
 * component()
 *   Creates the characters
 *   @param int width 
 *   @param int height 
 *   @param string color 
 *   @param int x 
 *   @param int y 
 *   @param string playerType 
 */
function component(width, height, color, x, y, playerType) {
    this.id = createCharacterId();
    this.playerType = playerType;
    this.width = width;
    this.height = height;
    this.lastMoveTime = (new Date()).getTime();
    this.x = x;
    this.y = y;
    this.active = true;
    this.update = function() {
        ctx = gameMat.context;
        ctx.fillStyle = color;
        ctx.fillRect(this.x, this.y, this.width, this.height);
    }
}

/**
 * updateGameArea()
 *   Updates the bots and players positions
 */
function updateGameArea() {
    gameMat.clear();
    if (character.active) {
        character.update();
    }
    bots.forEach(bot => {
        bot.update();
    });
    checkCollisions();
}

/**
 * removePlayers
 * Removes a list of players from the game.
 * @param list players : list of players to remove
 */
function removePlayers(players) {
    players.forEach(player => {
        if (player.playerType == "bot") {
            for (var i = 0; i < bots.length; i++) {
                if (bots[i].id == player.id) {
                    bots.splice(i, 1);
                    return true;
                }
            }
        } else if (player.playerType == "player") {
            character.active = false;
            alert("YOU LOSE!")
        }
    });
    
}

/**
 * checkCollisiions()
 * Checks to see if any players collided. 
 */
function checkCollisions() {
    var allPlayers = bots.slice(0);
    allPlayers.push(character);
    //console.log(allPlayers);
    for (var j = 0; j < allPlayers.length; j++) {
        var player = allPlayers[j];
        if (!player.active) {
            continue;
        }
        for (var i = 0; i < allPlayers.length; i++) {
            if (i == j || !allPlayers[i].active) {
                continue;
            }
            if (player.x == allPlayers[i].x && player.y == allPlayers[i].y) {
                if (player.lastMoveTime > allPlayers[i].lastMoveTime) {
                    removePlayers([allPlayers[i]]);
                    console.log(player.id + " collided with " + allPlayers[i].id + ". " + allPlayers[i].id + " is out");
                }
                if (player.lastMoveTime == allPlayers[i].lastMoveTime) {
                    removePlayers([allPlayers[i], player]);
                    console.log(player.id + " collided with " + allPlayers[i].id + ". " + "Both players are out!");
                }
            }
        }
    }
}

/**
 * addListener()
 *   Adds the keydown listener for the player movement
 */
function addListener() {
    document.addEventListener("keydown", function(e) {
        var moved = false;
        switch(e.key) {
            case "w":
                if (character.y - 25 >= 0) {
                    character.y -= 25;
                    moved = true;
                }
                break;
            case "s":
                if (character.y + 25 <= gameMat.canvas.height - 25) {
                    character.y += 25;
                    moved = true;
                }
                break;
            case "a":
                if (character.x - 25 >= 0) {
                    character.x -= 25;
                    moved = true;
                }
                break;
            case "d":
                if (character.x + 25 <= gameMat.canvas.width - 25) {
                    character.x += 25;
                    moved = true;
                }
                break;            
            default:
                break;
        }
        if (moved) { 
            sendLocation(character.id, character.x, character.y);
            character.lastMoveTime = (new Date()).getTime();
        }
    })
}


/**
 * createCharacterId
 * Creates a UUID for a character.
 */
function createCharacterId(){
    var dt = new Date().getTime();
    var uuid = 'xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
        var r = (dt + Math.random()*16)%16 | 0;
        dt = Math.floor(dt/16);
        return (c=='x' ? r :(r&0x3|0x8)).toString(16);
    });
    return uuid;
}

