const { invoke } = window.__TAURI__.tauri;

invoke('test_func', { name: 'PLEASE' }).then((res) => {
  window.header.innerHTML = res;
})

const btn = document.getElementById('fetch-button');

btn.onclick = () => {
  invoke('btn_click');
}