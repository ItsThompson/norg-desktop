//import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

//let greetInputEl: HTMLInputElement | null;
//let greetMsgEl: HTMLElement | null;

async function readFileContents() {
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
    console.log(selectedPath);
  } catch (err) {
    console.error(err);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  //greetInputEl = document.querySelector("#greet-input");
  //greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#file-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    readFileContents();
  });
});
