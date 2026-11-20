document.documentElement.classList.add('js');
const menu = document.querySelector('.menu-button');
const sidebar = document.querySelector('#sidebar');
menu?.addEventListener('click', () => {
  const open = menu.getAttribute('aria-expanded') !== 'true';
  menu.setAttribute('aria-expanded', String(open));
  sidebar.classList.toggle('is-open', open);
});
document.addEventListener('keydown', event => {
  if (event.key === 'Escape' && menu?.getAttribute('aria-expanded') === 'true') {
    menu.setAttribute('aria-expanded', 'false');
    sidebar.classList.remove('is-open');
    menu.focus();
  }
});
if (navigator.clipboard) {
  document.querySelectorAll('pre').forEach(pre => {
    const button = document.createElement('button');
    button.className = 'copy-button';
    button.type = 'button';
    button.textContent = 'Copy';
    button.setAttribute('aria-label', 'Copy code example');
    button.addEventListener('click', async () => {
      try {
        await navigator.clipboard.writeText(pre.querySelector('code').textContent);
        button.textContent = 'Copied';
      } catch {
        button.textContent = 'Select to copy';
      }
      setTimeout(() => { button.textContent = 'Copy'; }, 1800);
    });
    pre.append(button);
  });
}
