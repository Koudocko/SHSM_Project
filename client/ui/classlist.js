// Add event listeners to the edit and reset buttons
const editButtons = document.querySelectorAll('.edit-btn');
editButtons.forEach(button => {
    button.addEventListener('click', () => {
        // Get the current student information
        const name = button.parentNode.parentNode.querySelector('.name1').textContent;
        const username = button.parentNode.parentNode.querySelector('.username1').textContent;
        const password = button.parentNode.parentNode.querySelector('.password-text1').textContent;

        // Fill the form with the current student information
        document.querySelector('#edit-name').value = name;
        document.querySelector('#edit-username').value = username;
        document.querySelector('#edit-password').value = password;

        // Show the modal
        const modal = document.querySelector('#edit-modal');
        modal.style.display = 'block';

        // Add event listener to the save button in the edit modal
        const saveButton = document.querySelector('.save-btn');
        saveButton.addEventListener('click', event => {
            event.preventDefault();
            document.getElementsByClassName("name1")[0].textContent = document.querySelector("#edit-name").value;
            document.getElementsByClassName("username1")[0].textContent = document.querySelector("#edit-username").value;
            document.getElementsByClassName("password-text1")[0].textContent = document.querySelector("#edit-password").value;
        })
    });
});

// Event listener for clicking on add certification.
const addButton = document.querySelectorAll(".addcrt-btn");
addButton.forEach(button => {
    button.addEventListener('click', () => {
        const modal = document.querySelector("#add-modal");
        modal.style.display = 'block';
    });
});

// Event listener for clicking on add certification.
const eventButton = document.querySelectorAll(".event-btn");
eventButton.forEach(button => {
    button.addEventListener('click', () => {
        const modal = document.querySelector("#event-modal");
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


