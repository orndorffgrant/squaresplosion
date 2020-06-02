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

        bots.forEach(bot => {
            bot.color = "#FFFFFF";
        });
        document.getElementById("darkmodeSlider").classList.add("on");
    } else {
        //Disables dark mode
        body.classList.remove("darkmode");
        
        darkmode = false; //used in game.js
        bots.forEach(bot => {
            bot.color = "#000000";
        });
        document.getElementById("darkmodeSlider").classList.remove("on");
    }
}

//Toggles dark mode based on user preferences. 
if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
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
    window.getSelection().addRange(range);
    document.execCommand("copy");
}