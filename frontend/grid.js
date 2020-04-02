var character;
var gameMat;

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

function getRandomColor() {
    var letters = '0123456789ABCDEF';
    var color = '#';
    for (var i = 0; i < 6; i++) {
      color += letters[Math.floor(Math.random() * 16)];
    }
    return color;
  }

function startGame() {
    createGameMat();
    var randomX = Math.floor(Math.random() * ((gameMat.canvas.width / 25))) * 25;
    var randomY = Math.floor(Math.random() * ((gameMat.canvas.height / 25))) * 25;
    var randomColor = getRandomColor();
    console.log(randomColor);
    document.getElementById("grid").style.borderColor = randomColor;
    character = new component(25, 25, randomColor, randomX, randomY);
    addListener();
    gameMat.start();
}

function component(width, height, color, x, y) {
    this.width = width;
    this.height = height;
    this.speedX = 0;
    this.speedY = 0;
    this.x = x;
    this.y = y;
    this.update = function() {
        ctx = gameMat.context;
        ctx.fillStyle = color;
        ctx.fillRect(this.x, this.y, this.width, this.height);
    }
    this.newPos = function() {
        this.x = x + this.speedX;
        this.y = y + this.speedY;
    }
}

function updateGameArea() {
    gameMat.clear();
    character.newPos();
    character.update();
}

function addListener() {
    document.addEventListener("keypress", function(e) {
        switch(e.key) {
            case "w":
                if (character.y - 25 >= 0) {
                    character.speedY -= 25;
                }
                break;
            case "s":
                if (character.y + 25 <= gameMat.canvas.height - 25) {
                    character.speedY += 25;
                }
                break;
            case "a":
                if (character.x - 25 >= 0) {
                    character.speedX -= 25;
                }
                break;
            case "d":
                if (character.x + 25 <= gameMat.canvas.width - 25) {
                    character.speedX += 25;
                }
                break;
            default:
                break;
        }
    })
}

