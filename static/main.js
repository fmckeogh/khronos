/// s4202, wics, and cs enabled by default
var selectedGroups = new Set(["cs4202", "wics", "cs"]);

window.onload = async (_event) => {
  fetch("/groups")
    .then((response) => {
      return response.json();
    })
    .then((groups) => {
      for (const group of groups) {
        addButton(group);
      }
      setDefaultButtonStates();
      updateLink();
    });
};

/// We want cs4202, wics, and cs enabled by default to demonstrate to the user how it works
function setDefaultButtonStates() {
  selectedGroups.forEach((value) =>
    bootstrap.Button.getOrCreateInstance(
      document.getElementById(value)
    ).toggle()
  );
}

/// Adds a new button corresponding to an event group to the page
function addButton(group) {
  let button = document.createElement("button");
  button.type = "button";
  button.classList.add("btn");
  button.classList.add("btn-outline-primary");
  button.classList.add("m-2");
  button.setAttribute("data-bs-toggle", "button");
  button.id = group;
  button.textContent = group;
  button.onclick = buttonClick;
  document.getElementById(getGroupId(group)).appendChild(button);
}

/// If a button is clicked, update selectedGroups and the displayed
function buttonClick(evt) {
  let button = evt.target;
  if (button.classList.contains("active")) {
    selectedGroups.add(button.id);
  } else {
    selectedGroups.delete(button.id);
  }

  updateLink();
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

/// Updates the value of the webcal link being displayed
function updateLink() {
  let contents = "webcal://eventcal.uk/calendar/";
  let base_length = contents.length;

  selectedGroups.forEach((group) => {
    if (contents.length != base_length) {
      contents += "+";
    }

    contents += group;
  });

  let link = document.getElementById("link");
  link.innerText = contents;
  link.setAttribute("href", contents);
}
