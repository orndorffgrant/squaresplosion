/**
 * toggleDarkMode
 *   Toggles the dark mode based on the classes attached to the body. 
 */
function toggleDarkMode() {
    var body = document.getElementsByTagName("body")[0];
    var icons = document.getElementsByClassName('icon');
    if (!body.classList.contains("darkmode")) {
        //Enables dark mode
        body.classList.add("darkmode");
        
        darkmode = true; //used in game.js

        //bots.forEach(bot => {
        //    bot.color = "#FFFFFF";
        //});
        document.getElementById("darkmodeSlider").classList.add("on");
        localStorage.setItem("darkmode", "on");
    } else {
        //Disables dark mode
        body.classList.remove("darkmode");

        darkmode = false; //used in game.js
        //bots.forEach(bot => {
        //    bot.color = "#000000";
        //});
        document.getElementById("darkmodeSlider").classList.remove("on");
        localStorage.setItem("darkmode", "off");
    }
}

//Toggles dark mode based on user preferences.
if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
    document.getElementById("darkModeCheckbox").click();
}

//Toggles dark mode if they've turned it on before.
if (localStorage.getItem("darkmode") === "on") {
    document.getElementById("darkModeCheckbox").click();
}

window.matchMedia('(prefers-color-scheme: dark)').addListener(e => {
    if (e.matches) {
        if (!document.getElementById("darkModeCheckbox").checked) {
            document.getElementById("darkModeCheckbox").click();
        }
    }
});
window.matchMedia('(prefers-color-scheme: light)').addListener(e => {
    if (e.matches) {
        if (document.getElementById("darkModeCheckbox").checked) {
            document.getElementById("darkModeCheckbox").click();
        }
    }
});
window.matchMedia('(prefers-color-scheme: no-preference)').addListener(e => {
    if (e.matches) {
        if (document.getElementById("darkModeCheckbox").checked) {
            document.getElementById("darkModeCheckbox").click();
        }
    }
});
//End of dark mode toggling.

function copyToClipboard(id) {
    var range = document.createRange();
    range.selectNode(document.getElementById(id));
    window.getSelection().removeAllRanges();
    window.getSelection().addRange(range);
    document.execCommand("copy");
}

/**
 * Updates the leaderboard. Currently is called from websocket.js
 * @param obj players : the player states
 */
function updateLeaderboard(players) {
    var playerOrder = [];
    for (var key in players) {
        var color = "";
        if(character.id == key) {
            color = character.color;
        } else {
            for (var i = 0; i < otherPlayers.length; i++) {
                if (otherPlayers[i].id == key) {
                    color = otherPlayers[i].color;
                    break;
                }  
            }
        }
        playerOrder.push({"name": players[key].name, "score": players[key].score, "color": color});
    }
    playerOrder.sort((a, b) => parseFloat(b.score) - parseFloat(a.score));

    var leaderboard = document.getElementById("leaderboardBody");
    leaderboard.innerHTML = "";
    var template = document.getElementById("leaderboardRowTemplate")
    playerOrder.forEach(element => {
        var clone = template.content.cloneNode(true);
        var tr = clone.querySelector("tr");
        var tds = clone.querySelectorAll("td");
        tr.style.color = element.color;
        tds[0].textContent = element.name;
        tds[1].textContent = element.score;
        leaderboard.appendChild(clone);
    });
}