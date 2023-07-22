function toggleSubOptions(id) {
  const subOptions = document.getElementById('playlistsOptions');
  if (id === 'playlists') {
      subOptions.style.display = subOptions.style.display === "none" ? "block" : "none";
  }
}
