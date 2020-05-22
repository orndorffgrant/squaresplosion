var character;
var otherPlayers = [];
var bots = [];
var gameMat;
var darkmode = false;

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
    colors = ["#42f5e3", "#ae35d0", "#f542a4", "#f54242","#f59942", "#7ed537", "#42f5e3"];
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
    attemptConnection();
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
    botColor = "#000000";
    if (darkmode) {
        botColor = "#FFFFFF";
    }
    bot = new component(25, 25, botColor, botXY[0], botXY[1], "bot");
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
            sendLocation(bots[i].id, bots[i].x, bots[i].y); //Bots may move too fast for this.
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
function component(width, height, color, x, y, playerType, id = null) {
    if (id === null) {
        this.id = createCharacterId();
    } else {
        this.id = id;
    }
    this.playerType = playerType;
    this.color = color;
    this.width = width;
    this.height = height;
    this.lastMoveTime = (new Date()).getTime();
    this.x = x;
    this.y = y;
    this.active = true;
    this.update = function() {
        ctx = gameMat.context;
        ctx.fillStyle = this.color;
        ctx.fillRect(this.x, this.y, this.width, this.height);
    }
    this.totalExplodeParticles = (this.width * this.height);
    this.exploding = false;
    this.explodeParticles = [];
    this.explodeMiddle = [];
    this.getExplodeParticles = function() {
        this.exploding = true;
        var copyForMiddle = []; //I could not get a copy without reference, so this was the next best idea...
        for (var i = 0; i < width; i++) {
            for (var j = 0; j < height; j++) {
                /*
                    0 = x
                    1 = y
                    2 = length of explosion (in frames)
                    3 = Helps randomize explosion some. Change 4 to 3 to remove fading square
                */
                this.explodeParticles.push([this.x+i, this.y+j, Math.floor(Math.random() * 50), Math.floor(Math.random() * 4)]);
                copyForMiddle.push([this.x+i, this.y+j]);
            }
        }
        this.explodeMiddle = copyForMiddle[Math.floor((this.width * this.height)/2)];
    }
    this.explodeAnimation = function() {
        var hasParticles = false; //Helps turn off the animation when it is over.
        for (var i=0;i < this.explodeParticles.length; i++) {
            if (this.explodeParticles[i][0] == this.explodeMiddle[0] || this.explodeParticles[i][1] == this.explodeMiddle[1]) {
                continue;
            } else if (this.explodeParticles[i][2] == 0) {
                continue;
            }
            hasParticles = true;

            if (this.explodeParticles[i][3] == 0) {
                this.explodeMove("x", i);
                if (Math.floor(Math.random() * 2) === 0) { //Little bit of randomness to help it look better
                    this.explodeMove("y", i);
                }
            } else if (this.explodeParticles[i][3] == 1) {
                this.explodeMove("x", i);
                this.explodeMove("y", i);
                
            } else if (this.explodeParticles[i][3] == 2) {
                if (Math.floor(Math.random() * 2) === 0) { //Little bit of randomness to help it look better
                    this.explodeMove("x", i);
                }
                this.explodeMove("y", i);
            } else if (this.explodeParticles[i][3] == 3) {
                if (Math.floor(Math.random() * 25) === 0) {
                    this.explodeMove("x", i);
                }
                if (Math.floor(Math.random() * 25) === 0) {
                    this.explodeMove("y", i);
                }
            }

            this.explodeParticles[i][2]--;
            
            ctx = gameMat.context;
            ctx.fillStyle = this.color;
            ctx.fillRect(this.explodeParticles[i][0], this.explodeParticles[i][1], 1, 1);
        }
        if (!hasParticles) {
            this.exploding = false;
        }
    }
    this.explodeMove = function(direction, i) {
        if (direction == "x") {
            if (this.explodeParticles[i][0] < this.explodeMiddle[0]) {
                this.explodeParticles[i][0] = this.explodeParticles[i][0] - Math.floor(Math.random() * 5);
            } else if (this.explodeParticles[i][0] > this.explodeMiddle[0]) {
                this.explodeParticles[i][0] = this.explodeParticles[i][0] + Math.floor(Math.random() * 5);
            }
        } else if (direction == "y") {
            if (this.explodeParticles[i][1] < this.explodeMiddle[1]) {
                this.explodeParticles[i][1] = this.explodeParticles[i][1] - Math.floor(Math.random() * 5);
            } else if (this.explodeParticles[i][1] > this.explodeMiddle[1]) {
                this.explodeParticles[i][1] = this.explodeParticles[i][1] + Math.floor(Math.random() * 5);
            }
        }
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
    } else if (!character.exploding && character.explodeParticles.length == 0) {
        character.getExplodeParticles();
    } else if (character.exploding && character.explodeParticles.length != 0) {
        character.explodeAnimation();
    }
    bots.forEach(bot => {
        if (bot.active) {
            bot.update();
        } else if (!bot.exploding && bot.explodeParticles.length == 0) {
            bot.getExplodeParticles();
        } else if (bot.exploding && bot.explodeParticles.length  != 0){
            bot.explodeAnimation();
        }
    });
    otherPlayers.forEach(player => {
        if (player.active) {
            player.update();
        } else if (!player.exploding && player.explodeParticles.length == 0) {
            player.getExplodeParticles();
        } else if (player.exploding && player.explodeParticles.length  != 0){
            player.explodeAnimation();
        }
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
                    bots[i].active = false;
                    return true;
                }
            }
        } else if (player.playerType == "player") {
            character.active = false;
        } else if (player.playerType == "otherPlayer") {
            for (var i = 0; i < otherPlayers.length; i++) {
                if (otherPlayers[i].id == player.id) {
                    otherPlayers[i].active = false;
                    return true;
                }
            }
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
    allPlayers = allPlayers.concat(otherPlayers);
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
            case "ArrowUp":
            case "w":
                if (character.y - 25 >= 0) {
                    character.y -= 25;
                    moved = true;
                }
                break;
            case "ArrowDown":
            case "s":
                if (character.y + 25 <= gameMat.canvas.height - 25) {
                    character.y += 25;
                    moved = true;
                }
                break;
            case "ArrowLeft":
            case "a":
                if (character.x - 25 >= 0) {
                    character.x -= 25;
                    moved = true;
                }
                break;
            case "ArrowRight":
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
            character.lastMoveTime = (new Date()).getTime();
            sendLocation(character.id, character.x, character.y);
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

/**
 * createOtherPlayer
 * @param string id : the player Id
 * @param int x : x location
 * @param int y : y location
 */
function createOtherPlayer(id, x, y) {
    var player = new component(25, 25, getRandomColor(), x, y, "otherPlayer", id);
    otherPlayers.push(player);
}

