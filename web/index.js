import * as pedals from "pedals";

const play_button = document.getElementById("play");
play_button.addEventListener("click", event => {
  pedals.beep();
});
