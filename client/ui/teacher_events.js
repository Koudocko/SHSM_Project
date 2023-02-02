$(function () {
    $(document).scroll(function () {
        var $nav = $("#mainNavbar");
        $nav.toggleClass("scrolled", $(this).scrollTop() > $nav.height());
    });
});

document.getElementsByClassName("event-container")[0].addEventListener("click", function(e){
    const { invoke } = window.__TAURI__.tauri 

    var _title = e.target.parentNode.querySelector(".event-title").textContent;
    var _date = e.target.parentNode.querySelector(".event-date").textContent;
    var _description = e.target.parentNode.querySelector(".event-description").textContent;
    var _certification = e.target.parentNode.querySelector(".event-certified").textContent;
    _certification = _certification.substr(_certification.indexOf(" ") + 1);;

    if (e.target.className == "view-signups"){
        const modal = document.querySelector("#view-modal");
        modal.style.display = 'block';

        invoke('get_event_users', { title: _title })
          .then(() =>{});
    }
    else if (e.target.className == "edit-event"){
        document.querySelector("#event_name_e").value = _title;
        console.log(_date);
        document.querySelector("#event_date_e").value = _date;
        document.querySelector("#event_description_e").value = _description;
        console.log(_certification);
        document.querySelector("#event_certified_e").checked = _certification == 'true';

        const modal = document.querySelector("#edit-modal");
        modal.style.display = 'block';

        const saveButton2 = document.querySelector('.save-btn-2');
        saveButton2.addEventListener('click', event => {
            event.preventDefault();

            var _new_name = document.querySelector("#event_name_e").value;
            var _new_description = document.querySelector("#event_description_e").value;
            var _new_date = document.querySelector("#event_date_e").value;
            var _new_certification = document.querySelector("#event_certified_e").checked;

            invoke('update_event', { title: _title, newTitle: _new_name, newDescription: _new_description, newDate: _new_date, newCertification: _new_certification })
              .then(() =>{
                  location.reload();
              });
        });

        const removeButton = document.querySelector('.remove');
        removeButton.addEventListener('click', event => {
            invoke('remove_event', { title: _title })
              .then(() =>{
                  location.reload();
              });
        });
    }
}); 

const addButton = document.querySelector("#add-event-btn");
addButton.addEventListener('click', () => {
    const modal = document.querySelector("#add-event-modal");
    modal.style.display = 'block';
});

const saveButton1 = document.querySelector('.save-btn-1');
saveButton1.addEventListener('click', event => {
    console.log("save button clicked");
    const { invoke } = window.__TAURI__.tauri 
    event.preventDefault();

    var _title = document.getElementById("event_name").value;
    var _description = document.getElementById("event_description").value;
    var _date = document.getElementById("event_date").value;
    var _certification = document.getElementById("event_certified").checked;

    invoke('add_shsm_event', { title: _title, description: _description, date: _date, certification: _certification })
      .then(() =>{});
});

// Add event listener to the close buttons in the modals
const closeButtons = document.querySelectorAll('.close-btn');
closeButtons.forEach(button => {
    button.addEventListener('click', () => {
        const modal = button.parentNode.parentNode;
        modal.style.display = 'none';
    });
});
