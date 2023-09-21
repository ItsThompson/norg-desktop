import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

//let greetInputEl: HTMLInputElement | null;
//let greetMsgEl: HTMLElement | null;
let agendaDiv: HTMLElement | null;

let path: string | null;

function generateHTML(agenda: Array<Object>) {
  //todo: loop through agenda and output...
  for (let i = 0; i < agenda.length; i++) {
    if (agenda[i].hasOwnProperty("date")) {
      let dateString = (agenda[i] as any).date;
      let dateh1 = document.createElement("h1");
      dateh1.textContent = dateString;
      agendaDiv?.appendChild(dateh1);
    }
    if (agenda[i].hasOwnProperty("items")) {
        let ul = document.createElement("ul");
        for (let j = 0; j < (agenda[i] as any).items.length; j++) {
            let li = document.createElement("li");
            let itemString = (agenda[i] as any).items[j];
            li.textContent = itemString;
            ul?.appendChild(li);
        }
        agendaDiv?.appendChild(ul);
    }
  }
}

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
  if (agendaDiv && path) {
    let agenda: Array<Object> = await invoke("generate_text", {
      path,
    });

    console.debug(agenda);
    generateHTML(agenda);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  //greetInputEl = document.querySelector("#greet-input");
  //greetMsgEl = document.querySelector("#greet-msg");
  agendaDiv = document.querySelector("#agenda");
  document.querySelector("#file-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    getPath();
  });
});
