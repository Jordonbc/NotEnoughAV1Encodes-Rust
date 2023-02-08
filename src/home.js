import * as GLOBALS from '/global_variables.js';

function update_ui_elements() {
  //GLOBALS.project_directory_input_element.value = GLOBALS.project_directory;

}

function update_backend() {
  //GLOBALS.invoke("set_client_configuration", { newClientConfig: config });
}

window.addEventListener("DOMContentLoaded", () => {
  console.log("Running home.js");
  //GLOBALS.project_directory_input_button.addEventListener("click", () => GLOBALS.invoke("open_project_directory_dialog"));

  //GLOBALS.client_win64_element.addEventListener("click", () => update_backend());
  
  document.addEventListener("reload_ui", e => update_ui_elements());
});