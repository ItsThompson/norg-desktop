import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

//let greetInputEl: HTMLInputElement | null;
//let greetMsgEl: HTMLElement | null;
let testEl: HTMLElement | null;

let path: string | null;

async function getPath() {
  //if (greetMsgEl && greetInputEl) {
  //  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //  greetMsgEl.textContent = await invoke("greet", {
  //    name: greetInputEl.value,
  //  });
  //}

  try {
    const selectedPath = await open({
      directory: true,
      multiple: false,
      title: "Open Norg .yml File",
    });

    if (typeof selectedPath === "string") {
      path = selectedPath;
    }
  } catch (err) {
    console.error(err);
  }
  generateText(); //TODO figure out how to put this in window event listener and run after async function getPath finishes
}

async function generateText() {
  if (testEl && path) {
    let agenda: Array<Object> = await invoke("generate_text", {
      path,
    });

    console.debug(agenda);
    //todo: loop through agenda and output...

    let out: string = `test\n test`; //TODO figure out how to implement headers not just content
    testEl.textContent = out;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  //greetInputEl = document.querySelector("#greet-input");
  //greetMsgEl = document.querySelector("#greet-msg");
  testEl = document.querySelector("#test");
  document.querySelector("#file-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    getPath();
  });
});
