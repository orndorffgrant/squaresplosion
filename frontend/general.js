function toggleDarkMode() {
    if (!document.getElementById("darkmodecss")) {
        enableDarkMode();
    } else {
        disableDarkMode();
    }
}

function enableDarkMode() {
    var link = document.createElement("link");
    link.rel = "stylesheet";
    link.type = "text/css";
    link.href = "darkmode.css";
    link.id = "darkmodecss";
    document.getElementsByTagName("HEAD")[0].appendChild(link);
    darkmode = true; //used in game.js

    bots.forEach(bot => {
        bot.color = "#FFFFFF";
    });
    document.getElementById("darkmodeSlider").classList.add("on");
}

function disableDarkMode() {
    document.getElementById("darkmodecss").remove();
    darkmode = false;
    bots.forEach(bot => {
        bot.color = "#000000";
    });
    document.getElementById("darkmodeSlider").classList.remove("on");
}