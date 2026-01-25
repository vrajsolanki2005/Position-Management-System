// Dashboard functionality
const token = localStorage.getItem('access_token');

// Check authentication
if (!token) {
    window.location.href = '/';
    return;
}

// Verify token is valid
fetch('/api/user-stats', {
    headers: {
        'Authorization': `Bearer ${token}`
    }
})
.then(response => {
    if (!response.ok) {
        localStorage.removeItem('access_token');
        localStorage.removeItem('username');
        window.location.href = '/';
        return;
    }
    // Token is valid, start dashboard functionality
    initializeDashboard();
})
.catch(() => {
    localStorage.removeItem('access_token');
    localStorage.removeItem('username');
    window.location.href = '/';
});

function initializeDashboard() {
    // Auto-refresh leaderboard every 30 seconds
    setInterval(updateLeaderboard, 30000);
    
    // Update user stats every 10 seconds
    setInterval(updateUserStats, 10000);
    
    // Initial load
    updateLeaderboard();
    updateUserStats();
    
    // Initialize buttons after dashboard loads
    initializeButtons();
}

function initializeButtons() {
    const themeToggle = document.getElementById('theme-toggle');
    const logoutBtn = document.getElementById('logout-btn');
    const currentTheme = localStorage.getItem('theme') || 'dark';
    
    // Apply saved theme
    if (currentTheme === 'light') {
        document.body.classList.add('light-theme');
        if (themeToggle) themeToggle.innerHTML = '<i class="fas fa-sun"></i> Theme';
    }
    
    // Theme toggle functionality
    if (themeToggle) {
        themeToggle.onclick = function() {
            document.body.classList.toggle('light-theme');
            const isLight = document.body.classList.contains('light-theme');
            
            if (isLight) {
                localStorage.setItem('theme', 'light');
                themeToggle.innerHTML = '<i class="fas fa-sun"></i> Theme';
            } else {
                localStorage.setItem('theme', 'dark');
                themeToggle.innerHTML = '<i class="fas fa-moon"></i> Theme';
            }
        };
    }
    
    // Logout functionality
    if (logoutBtn) {
        logoutBtn.onclick = function() {
            localStorage.removeItem('access_token');
            localStorage.removeItem('username');
            window.location.href = '/';
        };
    }
}

function updateLeaderboard() {
    fetch('/api/leaderboard')
        .then(response => response.json())
        .then(data => {
            const leaderboardContainer = document.querySelector('.leaderboard');
            if (leaderboardContainer) {
                leaderboardContainer.innerHTML = data.map(user => `
                    <div class="leaderboard-item">
                        <span class="rank">#${user.rank}</span>
                        <span class="username">${user.username}</span>
                        <span class="score">${user.score}</span>
                    </div>
                `).join('');
            }
        })
        .catch(error => console.error('Error updating leaderboard:', error));
}

function updateUserStats() {
    fetch('/api/user-stats', {
        headers: {
            'Authorization': `Bearer ${token}`
        }
    })
    .then(response => response.json())
    .then(data => {
        // Update score display
        const scoreElement = document.querySelector('.stat-card h3');
        if (scoreElement) {
            scoreElement.textContent = data.score;
        }
        
        // Update labs completed
        const labsElement = document.querySelectorAll('.stat-card h3')[1];
        if (labsElement) {
            labsElement.textContent = data.labs_completed;
        }
    })
    .catch(error => console.error('Error updating user stats:', error));
}

// Direct button event listeners
setTimeout(() => {
    const themeBtn = document.getElementById('theme-toggle');
    const logoutBtn = document.getElementById('logout-btn');
    
    if (themeBtn) {
        themeBtn.onclick = () => {
            document.body.classList.toggle('light-theme');
            const isLight = document.body.classList.contains('light-theme');
            
            if (isLight) {
                localStorage.setItem('theme', 'light');
                themeBtn.innerHTML = '<i class="fas fa-sun"></i> Theme';
            } else {
                localStorage.setItem('theme', 'dark');
                themeBtn.innerHTML = '<i class="fas fa-moon"></i> Theme';
            }
        };
    }
    
    if (logoutBtn) {
        logoutBtn.onclick = () => {
            localStorage.removeItem('access_token');
            localStorage.removeItem('username');
            window.location.href = '/';
        };
    }
}, 1000);

