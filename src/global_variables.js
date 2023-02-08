export const { invoke } = window.__TAURI__.tauri;
export const listen = window.__TAURI__.event.listen;

// elements


// global variables

const reload_ui_event = new Event('reload_ui');

async function reload_variables() {
    console.log("Reloading Variables!");
    //project_directory = await invoke("get_project_directory");
    output_directory = await invoke("get_compiled_output_directory");

    document.dispatchEvent(reload_ui_event);
  }

function init_buttons() {
    //project_directory_input_button = document.getElementById("open_project_directory_button");
}

function init_input_elements() {
}

window.addEventListener("DOMContentLoaded", () => {
    console.log("Initializing global variables!");

    init_buttons();
    init_input_elements();

    reload_variables();
    listen("update_frontend", function (string) {
        reload_variables();
    });
});