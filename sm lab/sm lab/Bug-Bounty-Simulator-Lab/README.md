# Bug Bounty Simulator Lab

A full-stack web application for practicing ethical hacking and vulnerability assessment in a safe, controlled environment.

## Features

- **Dark Mode Toggle**: Switch between light and dark themes
- **User Authentication**: Secure JWT-based login/register system
- **AI-Generated Challenges**: Dynamic vulnerability scenarios
- **Educational Focus**: Safe environment for learning security concepts

## Tech Stack

- **Frontend**: HTML5, CSS3, Vanilla JavaScript
- **Backend**: Python Flask
- **Database**: SQLite
- **Authentication**: JWT with bcrypt password hashing
- **AI Integration**: OpenAI API (optional)

## Setup Instructions

1. **Install Dependencies**:
   ```bash
   pip install -r requirements.txt

## 2FA Setup with TOTP

1. **Install Authenticator App**:
   - Google Authenticator (iOS/Android)
   - Authy (iOS/Android/Desktop)
   - Microsoft Authenticator

2. **Setup Process**:
   - Register new account
   - Scan QR code with authenticator app
   - Enter 6-digit code to complete setup
   - Use authenticator app for all future logins

3. **Features**:
   - Works offline (no internet required)
   - Codes change every 30 seconds
   - More secure than email/SMS OTP
   - Free forever
   ```

2. **Environment Setup**:
   ```bash
   copy .env.example .env
   # Edit .env file with your API keys
   ```

3. **Run the Application**:
   ```bash
   python app.py
   ```

4. **Access the Application**:
   Open your browser and navigate to `http://localhost:5000`

## Project Structure

```
Bug-Bounty-Simulator-Lab/
├── app.py                 # Main Flask application
├── requirements.txt       # Python dependencies
├── templates/            # HTML templates
│   ├── base.html
│   ├── index.html
│   └── dashboard.html
├── static/              # Static assets
│   ├── css/
│   │   └── style.css
│   └── js/
│       ├── theme.js
│       ├── auth.js
│       └── dashboard.js
└── README.md
```

## Usage

1. **Landing Page**: Register or login to access the lab
2. **Dashboard**: Generate new vulnerability challenges
3. **Practice**: Work through challenges to improve security skills

## Security Features

- Password hashing with bcrypt
- JWT token-based authentication
- Input validation and sanitization
- Secure session management

## Contributing

This is an educational project. Feel free to extend it with additional features like:
- More vulnerability types
- Progress tracking
- Difficulty levels
- Team challenges

## License

Educational use only. Please practice ethical hacking responsibly.