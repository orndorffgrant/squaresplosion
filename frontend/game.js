var character;
var otherPlayers = [];
var gameMat;
var darkmode = false;

/**
 * createGameMat
 *   Creates the game mat based on the size of the canvas.
 */
function createGameMat() {
  gameMat = {
    canvas: document.getElementById("grid"),
    start: function () {
      this.canvas.width = this.canvas.getAttribute("width");
      this.canvas.height = this.canvas.getAttribute("height");
      this.context = this.canvas.getContext("2d");
      this.interval = setInterval(updateGameArea, 50);
    },
    clear: function () {
      this.context.clearRect(0, 0, this.canvas.width, this.canvas.height);
    },
  };
}

/**
 * getRandomColor
 *   Gets a random color from a list of approved colors
 *   return string (the color)
 */
function getRandomColor() {
  var colors = [
    "#42f5e3",
    "#ae35d0",
    "#f542a4",
    "#f54242",
    "#f59942",
    "#7ed537",
    "#42f5e3",
  ];
  return colors[Math.floor(Math.random() * colors.length)];
}

/**
 * startGame()
 *   Start the game! Calls function to create active player and game mat
 */
function startGame() {
  createGameMat();
  var randomX = Math.floor(Math.random() * (gameMat.canvas.width / 25)) * 25;
  var randomY = Math.floor(Math.random() * (gameMat.canvas.height / 25)) * 25;
  var randomColor = getRandomColor();
  document.getElementById("grid").style.borderColor = randomColor;
  character = new component(25, 25, randomColor, randomX, randomY, "player");
  attemptConnection();
  addListener();
  gameMat.start();
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
  this.lastMoveTime = new Date().getTime();
  this.x = x;
  this.y = y;
  this.dir = "none";
  this.moveX = x;
  this.moveY = y;
  this.active = true;
  this.move = function () {
    var moved = false;
    var newX = this.moveX;
    var newY = this.moveY;
    switch (this.dir) {
      case "up":
        if (this.y - 25 >= 0) {
          newY = this.y - 25;
        }
        break;
      case "down":
        if (this.y + 25 <= gameMat.canvas.height - 25) {
          newY = this.y + 25;
        }
        break;
      case "left":
        if (this.x - 25 >= 0) {
          newX = this.x - 25;
        }
        break;
      case "right":
        if (this.x + 25 <= gameMat.canvas.width - 25) {
          newX = this.x + 25;
        }
        break;
      default:
        break;
    }
    moved = newX !== this.moveX || newY !== this.moveY;
    if (moved) {
      this.moveX = newX;
      this.moveY = newY;
      this.lastMoveTime = new Date().getTime();
      sendLocation(this.id, this.moveX, this.moveY);
    }
  };
  this.update = function () {
    ctx = gameMat.context;
    ctx.fillStyle = this.color;
    ctx.fillRect(this.x, this.y, this.width, this.height);
  };
  this.totalExplodeParticles = this.width * this.height;
  this.exploding = false;
  this.explodeParticles = [];
  this.explodeMiddle = [];
  this.getExplodeParticles = function () {
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
        this.explodeParticles.push([
          this.x + i,
          this.y + j,
          Math.floor(Math.random() * 50),
          Math.floor(Math.random() * 4),
        ]);
        copyForMiddle.push([this.x + i, this.y + j]);
      }
    }
    this.explodeMiddle =
      copyForMiddle[Math.floor((this.width * this.height) / 2)];
  };
  this.explodeAnimation = function () {
    var hasParticles = false; //Helps turn off the animation when it is over.
    for (var i = 0; i < this.explodeParticles.length; i++) {
      if (
        this.explodeParticles[i][0] == this.explodeMiddle[0] ||
        this.explodeParticles[i][1] == this.explodeMiddle[1]
      ) {
        continue;
      } else if (this.explodeParticles[i][2] == 0) {
        continue;
      }
      hasParticles = true;

      if (this.explodeParticles[i][3] == 0) {
        this.explodeMove("x", i);
        if (Math.floor(Math.random() * 2) === 0) {
          //Little bit of randomness to help it look better
          this.explodeMove("y", i);
        }
      } else if (this.explodeParticles[i][3] == 1) {
        this.explodeMove("x", i);
        this.explodeMove("y", i);
      } else if (this.explodeParticles[i][3] == 2) {
        if (Math.floor(Math.random() * 2) === 0) {
          //Little bit of randomness to help it look better
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
      ctx.fillRect(
        this.explodeParticles[i][0],
        this.explodeParticles[i][1],
        1,
        1
      );
    }
    if (!hasParticles) {
      this.exploding = false;
    }
  };
  this.explodeMove = function (direction, i) {
    if (direction == "x") {
      if (this.explodeParticles[i][0] < this.explodeMiddle[0]) {
        this.explodeParticles[i][0] =
          this.explodeParticles[i][0] - Math.floor(Math.random() * 5);
      } else if (this.explodeParticles[i][0] > this.explodeMiddle[0]) {
        this.explodeParticles[i][0] =
          this.explodeParticles[i][0] + Math.floor(Math.random() * 5);
      }
    } else if (direction == "y") {
      if (this.explodeParticles[i][1] < this.explodeMiddle[1]) {
        this.explodeParticles[i][1] =
          this.explodeParticles[i][1] - Math.floor(Math.random() * 5);
      } else if (this.explodeParticles[i][1] > this.explodeMiddle[1]) {
        this.explodeParticles[i][1] =
          this.explodeParticles[i][1] + Math.floor(Math.random() * 5);
      }
    }
  };
}

/**
 * updateGameArea()
 *   Updates the players positions
 */
function updateGameArea() {
  gameMat.clear();
  if (character.active) {
    character.move();
    character.update();
  } else if (!character.exploding && character.explodeParticles.length == 0) {
    character.getExplodeParticles();
    document.getElementById("respawn").classList.remove("hidden");
  } else if (character.exploding && character.explodeParticles.length != 0) {
    character.explodeAnimation();
  }
  otherPlayers.forEach((player) => {
    if (player.active) {
      player.update();
    } else if (!player.exploding && player.explodeParticles.length == 0) {
      player.getExplodeParticles();
    } else if (player.exploding && player.explodeParticles.length != 0) {
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
  players.forEach((player) => {
    if (player.playerType == "player") {
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
  const allPlayers = [character, ...otherPlayers];
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
          //   console.log(
          //     player.id +
          //       " collided with " +
          //       allPlayers[i].id +
          //       ". " +
          //       allPlayers[i].id +
          //       " is out"
          //   );
        }
        if (player.lastMoveTime == allPlayers[i].lastMoveTime) {
          removePlayers([allPlayers[i], player]);
          //   console.log(
          //     player.id +
          //       " collided with " +
          //       allPlayers[i].id +
          //       ". " +
          //       "Both players are out!"
          //   );
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
  var lastTouch;
  document.addEventListener("touchmove", function (e) {
    e.preventDefault();
    var currTouch = e.changedTouches[0];
    if (lastTouch) {
      var x = currTouch.screenX - lastTouch.screenX;
      var y = currTouch.screenY - lastTouch.screenY;
      var useX = Math.abs(x) > Math.abs(y);
      if (useX) {
        if (x < -10) {
          character.dir = "left";
        } else if (x > 10) {
          character.dir = "right";
        }
      } else {
        if (y < -10) {
          character.dir = "up";
        } else if (y > 10) {
          character.dir = "down";
        }
      }
    }
    lastTouch = currTouch;
  });
  document.addEventListener("touchend", function (e) {
    e.preventDefault();
    character.dir = "none";
  });
  document.addEventListener("keydown", function (e) {
    e.preventDefault();
    switch (e.key) {
      case "ArrowUp":
      case "w":
        character.dir = "up";
        break;
      case "ArrowDown":
      case "s":
        character.dir = "down";
        break;
      case "ArrowLeft":
      case "a":
        character.dir = "left";
        break;
      case "ArrowRight":
      case "d":
        character.dir = "right";
        break;
      default:
        break;
    }
  });
  document.addEventListener("keyup", function (e) {
    e.preventDefault();
    character.dir = "none";
  });
}

/**
 * createCharacterId
 * Creates a UUID for a character.
 */
function createCharacterId() {
  var dt = new Date().getTime();
  var uuid = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".replace(/[xy]/g, function (c) {
    var r = (dt + Math.random() * 16) % 16 | 0;
    dt = Math.floor(dt / 16);
    return (c == "x" ? r : (r & 0x3) | 0x8).toString(16);
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

function updatePlayerLocations(playerStates) {
  var playerKeys = Object.keys(playerStates);

  for (var i = 0; i < playerKeys.length; i++) {
    var player = playerStates[playerKeys[i]];
    var updatedPlayer = false;
    if (playerKeys[i] == character.id) {
      character.x = player.x;
      character.y = player.y;
      character.active = player.alive;
      updatedPlayer = true;
      continue;
    }
    for (var j = 0; j < otherPlayers.length; j++) {
      if (playerKeys[i] == otherPlayers[j].id) {
        otherPlayers[j].x = player.x;
        otherPlayers[j].y = player.y;
        otherPlayers[j].active = player.alive;
        updatedPlayer = true;
        break;
      }
    }
    if (!updatedPlayer) {
      createOtherPlayer(player.id, player.x, player.y);
    }
  }
}
