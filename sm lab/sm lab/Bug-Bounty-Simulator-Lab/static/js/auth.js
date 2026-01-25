// Authentication functionality
const tabBtns = document.querySelectorAll('.tab-btn');
const authForms = document.querySelectorAll('.auth-form');
const loginForm = document.getElementById('loginForm');
const registerForm = document.getElementById('registerForm');
const getStartedBtn = document.getElementById('get-started');

// Tab switching
tabBtns.forEach(btn => {
    btn.addEventListener('click', () => {
        const targetTab = btn.getAttribute('data-tab');
        
        tabBtns.forEach(b => b.classList.remove('active'));
        authForms.forEach(f => f.classList.remove('active'));
        
        btn.classList.add('active');
        document.getElementById(`${targetTab}-form`).classList.add('active');
    });
});

let currentUserId = null;

// Login form submission
loginForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const username = document.getElementById('login-username').value;
    const password = document.getElementById('login-password').value;
    
    try {
        const response = await fetch('/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ username, password })
        });
        
        const data = await response.json();
        
        if (response.ok) {
            localStorage.setItem('access_token', data.access_token);
            localStorage.setItem('username', data.username);
            showGetStartedButton();
        } else {
            showError(data.error || 'Login failed');
        }
    } catch (error) {
        showError('Login failed. Please try again.');
    }
});

// Register form submission
registerForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const username = document.getElementById('register-username').value;
    const email = document.getElementById('register-email').value;
    const password = document.getElementById('register-password').value;
    
    try {
        const response = await fetch('/register', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ username, email, password })
        });
        
        const data = await response.json();
        
        if (response.ok) {
            localStorage.setItem('access_token', data.access_token);
            localStorage.setItem('username', data.username);
            showGetStartedButton();
        } else {
            showError(data.error || 'Registration failed');
        }
    } catch (error) {
        showError('Registration failed. Please try again.');
    }
});

// Get Started button
getStartedBtn.addEventListener('click', () => {
    const token = localStorage.getItem('access_token');
    if (token) {
        window.location.href = '/dashboard';
    } else {
        alert('Please login first');
    }
});

function showGetStartedButton() {
    const authContainer = document.querySelector('.auth-container');
    const username = localStorage.getItem('username');
    
    authContainer.innerHTML = `
        <div class="success-state">
            <h3>Welcome, ${username}!</h3>
            <p>Ready to start your bug bounty journey?</p>
            <button id="get-started" class="cta-btn">Enter Lab Environment</button>
            <button onclick="logout()" class="resend-btn" style="margin-top: 1rem;">Logout</button>
        </div>
    `;
    
    // Re-attach event listener and auto-redirect
    document.getElementById('get-started').addEventListener('click', () => {
        window.location.href = '/dashboard';
    });
    
    // Auto-redirect after 2 seconds
    setTimeout(() => {
        window.location.href = '/dashboard';
    }, 2000);
}



function logout() {
    localStorage.removeItem('access_token');
    localStorage.removeItem('username');
    window.location.reload();
}

function showError(message) {
    const existingError = document.querySelector('.error-message');
    if (existingError) existingError.remove();
    
    const errorDiv = document.createElement('div');
    errorDiv.className = 'error-message';
    
    // Check if it's a success message
    if (message.includes('sent') || message.includes('successful')) {
        errorDiv.classList.add('success-message');
    }
    
    errorDiv.textContent = message;
    
    const activeForm = document.querySelector('.auth-form.active');
    activeForm.appendChild(errorDiv);
    
    setTimeout(() => errorDiv.remove(), 5000);
}



// Check if user is already logged in
if (localStorage.getItem('access_token')) {
    showGetStartedButton();
}