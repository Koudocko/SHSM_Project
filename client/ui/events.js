var eventButtons = document.querySelectorAll('.event-signup');
var form = document.querySelector('#event-signup-form');

eventButtons.forEach(function (button) {
  button.addEventListener('click', function (event) {
    var eventId = event.target.getAttribute('data-event-id');
    form.querySelector('#event-id').value = eventId;
  });
});

form.addEventListener('submit', function (event) {
  event.preventDefault();
  // Send the form data to the server using ajax
});