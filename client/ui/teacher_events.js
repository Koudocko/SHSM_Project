$(function () {
    $(document).scroll(function () {
        var $nav = $("#mainNavbar");
        $nav.toggleClass("scrolled", $(this).scrollTop() > $nav.height());
    });
});

const addButton = document.querySelector("#add-event-btn");
addButton.addEventListener('click', () => {
    const modal = document.querySelector("#add-event-modal");
    modal.style.display = 'block';
});

// Event listener for clicking on add certification.
const eventButton = document.querySelectorAll(".view-signups");
eventButton.forEach(button => {
    button.addEventListener('click', () => {
        const modal = document.querySelector("#view-modal");
        modal.style.display = 'block';
    });
});

// Add event listener to the close buttons in the modals
const closeButtons = document.querySelectorAll('.close-btn');
closeButtons.forEach(button => {
    button.addEventListener('click', () => {
        const modal = button.parentNode.parentNode;
        modal.style.display = 'none';
    });
});

function add_shsm_event(){
    console.log("add_shsm_event called!");
    var _username = document.getElementById("event_name").value;
    var _description = document.getElementById("event_description").value;
    var _date = document.getElementById("event_date").value;
    var _certification = document.getElementById("event_certified").checked;

    invoke('add_shsm_event', { username: _username, description: _description, date: _date, certification: _certification })
      .then(() =>{});
}
