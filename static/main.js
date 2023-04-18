window.onload = async (_event) => {
  fetch("/groups")
    .then((response) => {
      return response.json();
    })
    .then((groups) => {
      for (const group of groups) {
        addButton(group);
      }

      setLink("webcal://eventcal.uk/calendar/cs+wics+stacs+cs4202");
    });
};

function addButton(group) {
  let button = document.createElement("button");
  button.type = "button";
  button.classList.add("btn");
  button.classList.add("btn-outline-primary");
  button.classList.add("m-2");
  button.textContent = group;
  button.setAttribute("data-bs-toggle", "button");
  document.getElementById(getGroupId(group)).appendChild(button);
}

/// Gets the ID of the meta group of an event group
///
/// For example, "cs4099" and "cs4202" are modules, "wic"s is a society
function getGroupId(group) {
  switch (group) {
    case "cs":
      return "schools";
    case "stacs":
    case "wics":
      return "societies";
    default:
      return "modules";
  }
}

/// Sets the value of the webcal link being displayed
function setLink(contents) {
  let link = document.getElementById("link");
  link.innerText = contents;
  link.setAttribute("href", contents);
}

/// We want cs4202, wics, and cs enabled by default to demonstrate to the user how it works
function setDefaultButtonStates() {}
