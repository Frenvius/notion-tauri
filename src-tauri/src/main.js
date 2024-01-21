window.onload = () => {
    const iframe = document.getElementById('notionIframe');

    iframe.onload = () => {
        try {
            const shadowRoot = iframe.attachShadow({ mode: 'open' });

            const style = document.createElement('style');
            style.textContent = `
                /* Your custom CSS here */
                body {
                    /* Custom styles */
                }
            `;

            shadowRoot.appendChild(style);
        } catch (error) {
            console.error('Error injecting CSS:', error);
        }
    };
};