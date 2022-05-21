function createGistFromView(e) {
    e.preventDefault();
    let text = document.getElementById('gistText').value;
    let written = document.getElementById('written');

    if (text) {
        alert('Gist created!');
        written.innerHTML = text;
    }
}
