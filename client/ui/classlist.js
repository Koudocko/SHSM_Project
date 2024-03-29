// Add event listeners to the edit and reset buttons
document.getElementById("students-container").addEventListener("click", function(e){
    var _username = e.target.parentNode.parentNode.querySelector('.username1').textContent;

    if (e.target.className == "edit-btn"){
        // Fill the form with the current student information
        document.querySelector('#edit-username').value = _username;

        // Show the modal
        const modal = document.querySelector('#edit-modal');
        modal.style.display = 'block';

        // Add event listener to the save button in the edit modal
        const saveButton = document.querySelector('.save-btn');
        saveButton.addEventListener('click', event => {
            const { invoke } = window.__TAURI__.tauri 
            event.preventDefault();

            _username = e.target.parentNode.parentNode.querySelector('.username1').textContent;
            var _new_username = document.querySelector("#edit-username").value;
            var _new_password = document.querySelector("#edit-password").value;
            invoke('update_user', { username: _username, newUsername: _new_username, newPassword: _new_password })
              .then(() =>{
                  location.reload();
              });
        });
    }
    else if (e.target.className == "addcrt-btn"){
        const { invoke } = window.__TAURI__.tauri 
        const modal = document.querySelector("#add-modal");
        modal.style.display = 'block';

        invoke('get_user_events', { username: _username })
          .then(() =>{
            document.getElementById("certifications-container").addEventListener("click", function(f){
                if (f.target.className == "check"){
                    invoke('certify_user', { username: _username, certificationName: f.target.name, checked: f.target.checked })
                      .then(() =>{});
                }
            });
          });
    }
    else if (e.target.className == "rmv-btn"){
        const { invoke } = window.__TAURI__.tauri 

        const _username = e.target.parentNode.parentNode.querySelector('.username1').textContent;
        invoke('remove_user', { username: _username })
          .then(() =>{});
    }
});

// Add event listener to the close buttons in the modals
const closeButtons = document.querySelectorAll('.close-btn');
closeButtons.forEach(button => {
    button.addEventListener('click', () => {
        const modal = button.parentNode.parentNode;
        modal.style.display = 'none';
    });
});
