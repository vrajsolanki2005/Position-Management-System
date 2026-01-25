from flask import Flask, render_template, request, jsonify, redirect, url_for, session
from flask_sqlalchemy import SQLAlchemy
from flask_jwt_extended import JWTManager, create_access_token, jwt_required, get_jwt_identity, verify_jwt_in_request
from flask_mail import Mail, Message
from werkzeug.security import generate_password_hash, check_password_hash
from functools import wraps
import os
from datetime import datetime, timedelta, date
import openai
from dotenv import load_dotenv
import secrets
import re
from collections import defaultdict

load_dotenv()

app = Flask(__name__)
app.config['SECRET_KEY'] = 'your-secret-key-change-in-production'
app.config['SQLALCHEMY_DATABASE_URI'] = 'sqlite:///bug_bounty_lab.db'
app.config['SQLALCHEMY_TRACK_MODIFICATIONS'] = False
app.config['JWT_SECRET_KEY'] = 'jwt-secret-change-in-production'
app.config['JWT_ACCESS_TOKEN_EXPIRES'] = timedelta(hours=24)

# Flask-Mail configuration
app.config['MAIL_SERVER'] = 'smtp.gmail.com'
app.config['MAIL_PORT'] = 587
app.config['MAIL_USE_TLS'] = True
app.config['MAIL_USERNAME'] = os.getenv('SMTP_EMAIL')
app.config['MAIL_PASSWORD'] = os.getenv('SMTP_PASSWORD')
app.config['MAIL_DEFAULT_SENDER'] = os.getenv('SMTP_EMAIL')

db = SQLAlchemy(app)
jwt = JWTManager(app)
mail = Mail(app)

# Rate limiting storage
login_attempts = defaultdict(list)
register_attempts = defaultdict(list)

import random
import openai
import os
import requests
from vulnerability_db import VULNERABILITY_DATABASE

# Simple API-based AI Assistant
class SecurityMentorAI:
    def __init__(self):
        # API keys
        self.openai_key = os.getenv('OPENAI_API_KEY')
        self.gemini_key = os.getenv('GEMINI_API_KEY')
        
        if self.openai_key:
            openai.api_key = self.openai_key
            print("🤖 OpenAI API ready!")
        elif self.gemini_key:
            print("🚀 Gemini API ready!")
        else:
            print("⚡ Using local AI")
        
        self.personality_responses = [
            "🔒 Hey there, security explorer! ",
            "⚡ Ready to dive into cybersecurity? ",
            "🛡️ Security question incoming! ",
            "🎯 Let's hack some knowledge! ",
            "💻 Cyber detective mode activated! "
        ]
        
        self.vulnerability_database = VULNERABILITY_DATABASE
    
    def generate_response(self, user_input):
        # Try OpenAI API first
        if self.openai_key:
            try:
                response = openai.ChatCompletion.create(
                    model="gpt-3.5-turbo",
                    messages=[
                        {"role": "system", "content": "You are Mohanlal, an expert cybersecurity instructor. Answer questions directly and practically. If asked about vulnerabilities, provide specific examples, payloads, and step-by-step exploitation techniques. If asked about tools, explain exact usage. Be detailed and technical."},
                        {"role": "user", "content": user_input}
                    ],
                    max_tokens=800,
                    temperature=0.1
                )
                return response.choices[0].message.content.strip()
            except Exception as e:
                print(f"OpenAI error: {e}")
        
        # Try Gemini API
        if self.gemini_key:
            try:
                response = requests.post(
                    f"https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={self.gemini_key}",
                    json={
                        "contents": [{
                            "parts": [{"text": f"You are Mohanlal, a cybersecurity expert. Give a detailed, technical answer to: {user_input}. Include specific examples, code, or commands when relevant."}]
                        }]
                    }
                )
                if response.status_code == 200:
                    result = response.json()
                    return result['candidates'][0]['content']['parts'][0]['text']
            except Exception as e:
                print(f"Gemini error: {e}")
        
        # Enhanced local responses
        return self._get_smart_response(user_input)
    
    def _get_smart_response(self, user_input):
        user_lower = user_input.lower().strip()
        
        # Direct vulnerability questions
        if 'xss' in user_lower:
            if 'payload' in user_lower or 'example' in user_lower:
                return "XSS Payloads:\n<script>alert('XSS')</script>\n<img src=x onerror=alert(1)>\n<svg onload=alert(1)>\n<iframe src=javascript:alert(1)>\n\nFor bypassing filters:\n<ScRiPt>alert(1)</ScRiPt>\n<img src=x onerror=\"alert(1)\">\n<svg/onload=alert(1)>"
            elif 'prevent' in user_lower:
                return "XSS Prevention:\n1. Input validation and sanitization\n2. Output encoding (HTML entities)\n3. Content Security Policy (CSP)\n4. Use frameworks with auto-escaping\n5. Validate on server-side\n\nExample CSP: Content-Security-Policy: default-src 'self'"
            else:
                return "XSS (Cross-Site Scripting) Types:\n1. Reflected: Payload in URL, executes immediately\n2. Stored: Payload saved in database, executes for all users\n3. DOM-based: Client-side script manipulation\n\nTest with: <script>alert(1)</script>"
        
        elif 'sql injection' in user_lower or 'sqli' in user_lower:
            if 'payload' in user_lower or 'example' in user_lower:
                return "SQL Injection Payloads:\n\nAuthentication Bypass:\n' OR '1'='1' --\nadmin'--\n' OR 1=1#\n\nUNION Attacks:\n' UNION SELECT 1,2,3--\n' UNION SELECT username,password FROM users--\n\nBlind SQLi:\n' AND 1=1--  (True)\n' AND 1=2--  (False)\n' AND SLEEP(5)--  (Time delay)"
            elif 'prevent' in user_lower:
                return "SQL Injection Prevention:\n1. Use parameterized queries/prepared statements\n2. Input validation and sanitization\n3. Least privilege database access\n4. Stored procedures\n5. Escape special characters\n\nExample: SELECT * FROM users WHERE id = ? (parameterized)"
            else:
                return "SQL Injection Types:\n1. UNION-based: Extract data directly\n2. Boolean Blind: True/false responses\n3. Time-based: Delay responses\n4. Error-based: Database error messages\n\nBasic test: ' OR '1'='1' --"
        
        elif 'csrf' in user_lower:
            if 'payload' in user_lower or 'example' in user_lower:
                return "CSRF Attack Examples:\n\nGET Request:\n<img src=\"http://bank.com/transfer?to=attacker&amount=1000\">\n\nPOST Request:\n<form action=\"http://bank.com/transfer\" method=\"POST\">\n<input name=\"to\" value=\"attacker\">\n<input name=\"amount\" value=\"1000\">\n</form>\n<script>document.forms[0].submit()</script>"
            else:
                return "CSRF (Cross-Site Request Forgery):\nTricks users into performing unwanted actions\n\nPrevention:\n1. CSRF tokens in forms\n2. SameSite cookies\n3. Referrer validation\n4. Double submit cookies\n\nTest: Remove CSRF token from forms"
        
        elif 'burp' in user_lower or 'burp suite' in user_lower:
            return "Burp Suite Usage:\n1. Set browser proxy to 127.0.0.1:8080\n2. Intercept requests in Proxy tab\n3. Send to Repeater for manual testing\n4. Use Intruder for automated attacks\n5. Scanner for vulnerability detection\n\nKey shortcuts:\nCtrl+R: Send to Repeater\nCtrl+I: Send to Intruder"
        
        elif 'sqlmap' in user_lower:
            return "SQLMap Commands:\n\nBasic scan:\nsqlmap -u \"http://site.com/page?id=1\"\n\nDatabase enumeration:\nsqlmap -u \"url\" --dbs\nsqlmap -u \"url\" -D database --tables\nsqlmap -u \"url\" -D db -T table --dump\n\nPOST data:\nsqlmap -u \"url\" --data=\"id=1&name=test\""
        
        elif 'nmap' in user_lower:
            return "Nmap Commands:\n\nBasic scan:\nnmap target.com\n\nService detection:\nnmap -sV target.com\n\nScript scan:\nnmap -sC target.com\n\nAll ports:\nnmap -p- target.com\n\nStealth scan:\nnmap -sS target.com"
        
        # Greeting
        elif any(word in user_lower for word in ['hello', 'hi', 'hey']):
            return "Hello! I'm Mohanlal, your cybersecurity expert. Ask me specific questions about:\n- XSS payloads and prevention\n- SQL injection techniques\n- CSRF attacks\n- Security tools (Burp, SQLMap, Nmap)\n- Any vulnerability exploitation"
        
        elif 'roadmap' in user_lower or 'learning path' in user_lower:
            return "🛣️ CYBERSECURITY ROADMAP:\n\n📚 BEGINNER (1-3 months):\n• Learn networking basics (TCP/IP, DNS, HTTP)\n• Understand web technologies (HTML, JS, databases)\n• Basic Linux commands\n• Install Kali Linux\n\n🔧 INTERMEDIATE (3-6 months):\n• OWASP Top 10 vulnerabilities\n• Burp Suite, Nmap, Metasploit\n• Web application testing\n• Network scanning\n• Vulnerability assessment\n\n🎯 ADVANCED (6+ months):\n• Penetration testing methodology\n• Advanced exploitation techniques\n• Malware analysis\n• Incident response\n• Bug bounty hunting\n\n🏆 CERTIFICATIONS:\n• CEH (Ethical Hacker)\n• OSCP (Penetration Testing)\n• CISSP (Security Professional)"
        
        elif 'reconnaissance' in user_lower or 'recon' in user_lower:
            return "🔍 RECONNAISSANCE GUIDE:\n\n🎯 PASSIVE RECON (No direct contact):\n• Google Dorking: site:target.com filetype:pdf\n• Shodan.io: Find exposed services\n• theHarvester: Email/subdomain gathering\n• Maltego: OSINT framework\n• Social media intelligence\n\n⚡ ACTIVE RECON (Direct scanning):\n• Nmap: Port scanning\n  - nmap -sV -sC target.com\n  - nmap -p- target.com\n• Dirb/Gobuster: Directory enumeration\n• Nikto: Web vulnerability scanner\n• Whatweb: Technology identification\n\n📋 RECON METHODOLOGY:\n1. Information Gathering\n2. Subdomain Enumeration\n3. Port Scanning\n4. Service Enumeration\n5. Vulnerability Identification\n6. Exploitation Planning"
        
        elif 'tools' in user_lower and 'hacking' in user_lower:
            return "🛠️ ESSENTIAL HACKING TOOLS:\n\n🔍 RECONNAISSANCE:\n• Nmap - Port scanning\n• Masscan - Fast port scanner\n• theHarvester - Email/domain info\n• Shodan - Internet-connected devices\n• Maltego - OSINT framework\n\n🌐 WEB TESTING:\n• Burp Suite - Web proxy/scanner\n• OWASP ZAP - Free web scanner\n• Dirb/Gobuster - Directory brute force\n• Nikto - Web vulnerability scanner\n• SQLMap - SQL injection tool\n\n💥 EXPLOITATION:\n• Metasploit - Exploitation framework\n• Cobalt Strike - Advanced red teaming\n• Empire - PowerShell post-exploitation\n• Mimikatz - Windows credential extraction\n\n🔧 UTILITIES:\n• Wireshark - Network analysis\n• John the Ripper - Password cracking\n• Hashcat - Advanced password recovery\n• Hydra - Login brute forcer"
        
        elif 'penetration testing' in user_lower or 'pentest' in user_lower:
            return "🎯 PENETRATION TESTING METHODOLOGY:\n\n1️⃣ PLANNING & SCOPING:\n• Define scope and rules of engagement\n• Get written authorization\n• Identify target systems\n\n2️⃣ RECONNAISSANCE:\n• Passive information gathering\n• Active scanning and enumeration\n• Identify attack vectors\n\n3️⃣ VULNERABILITY ASSESSMENT:\n• Automated vulnerability scanning\n• Manual testing and validation\n• Risk assessment and prioritization\n\n4️⃣ EXPLOITATION:\n• Exploit identified vulnerabilities\n• Gain initial access\n• Privilege escalation\n\n5️⃣ POST-EXPLOITATION:\n• Maintain persistence\n• Lateral movement\n• Data exfiltration simulation\n\n6️⃣ REPORTING:\n• Executive summary\n• Technical findings\n• Remediation recommendations\n• Risk ratings and impact analysis"
        
        elif 'bug bounty' in user_lower:
            return "💰 BUG BOUNTY HUNTING GUIDE:\n\n🎯 GETTING STARTED:\n• Choose platforms: HackerOne, Bugcrowd, Synack\n• Read program policies carefully\n• Start with easier targets\n• Focus on one vulnerability type initially\n\n🔍 METHODOLOGY:\n1. Subdomain enumeration (Subfinder, Amass)\n2. Port scanning (Nmap, Masscan)\n3. Technology identification (Wappalyzer)\n4. Directory/file discovery (Ffuf, Dirsearch)\n5. Parameter discovery (Arjun, ParamSpider)\n6. Vulnerability testing (Manual + automated)\n\n💡 HIGH-IMPACT VULNS:\n• Authentication bypass\n• SQL injection\n• Remote code execution\n• IDOR (Insecure Direct Object Reference)\n• Business logic flaws\n\n📝 REPORTING TIPS:\n• Clear proof of concept\n• Step-by-step reproduction\n• Impact explanation\n• Suggested remediation"
        
        elif 'osint' in user_lower:
            return "🕵️ OSINT (Open Source Intelligence):\n\n🔍 SEARCH TECHNIQUES:\n• Google Dorks:\n  - site:target.com filetype:pdf\n  - inurl:admin site:target.com\n  - intitle:\"index of\" site:target.com\n\n🌐 DOMAIN INTELLIGENCE:\n• Whois lookup: whois target.com\n• DNS enumeration: dig target.com\n• Subdomain finder: subfinder -d target.com\n• Certificate transparency: crt.sh\n\n👥 SOCIAL MEDIA OSINT:\n• LinkedIn employee enumeration\n• Twitter/Facebook reconnaissance\n• GitHub code repository analysis\n• Pastebin data leaks\n\n🛠️ OSINT TOOLS:\n• theHarvester - Email/subdomain gathering\n• Maltego - Visual link analysis\n• Shodan - Internet-connected devices\n• Censys - Internet scanning\n• SpiderFoot - Automated OSINT"
        
        elif 'project' in user_lower or 'platform' in user_lower or 'lab' in user_lower:
            return "🚀 BUG BOUNTY SIMULATOR LAB:\n\n🎯 WHAT IS IT?\n• Cybersecurity training platform\n• 12 vulnerability labs for practice\n• Safe legal learning environment\n\n🏆 FOR WHO?\n• Students, bug hunters, pentesters, IT pros\n\nℹ️ More info: Ask 'vulnerabilities', 'features', or 'vulnshop'"
        
        elif 'vulnshop' in user_lower:
            return "🛍️ VULNSHOP - PRACTICE ENVIRONMENT:\n\n🎯 PURPOSE:\nA realistic e-commerce website intentionally designed with security vulnerabilities for educational penetration testing.\n\n💻 FEATURES:\n• 16 Products with real images\n• Shopping cart functionality\n• User reviews and ratings\n• Search functionality\n• Product categories\n• User-friendly interface\n\n🔓 PRACTICE VULNERABILITIES:\n• SQL Injection in login forms\n• XSS in search and reviews\n• CSRF in user actions\n• XXE in data processing\n• IDOR in user access\n• Command injection scenarios\n• File upload security issues\n\n🎮 HOW TO USE:\n1. Access from vulnerability labs\n2. Browse products normally\n3. Try to find security flaws\n4. Practice ethical exploitation\n5. Submit discovered flags\n6. Learn from guidance provided\n\n💡 LEARNING OBJECTIVES:\n• Understand real-world web security\n• Practice manual testing techniques\n• Learn vulnerability identification\n• Master safe exploitation methods\n• Develop security awareness"
        
        elif 'features' in user_lower:
            return "✨ PLATFORM FEATURES:\n\n🔐 USER SYSTEM:\n• Secure registration and login\n• Password protection\n• Account security features\n• User profiles and progress\n\n🏆 GAMIFICATION:\n• Point-based scoring system\n• Real-time leaderboard\n• Progress tracking\n• Difficulty-based rewards\n• Competition elements\n\n🤖 AI ASSISTANT (MOHANLAL):\n• Real-time chat support\n• Cybersecurity expertise\n• Step-by-step guidance\n• Vulnerability explanations\n• Learning assistance\n\n📊 DASHBOARD:\n• User statistics overview\n• Lab completion tracking\n• Performance metrics\n• Activity monitoring\n• Ranking system\n\n🎨 USER INTERFACE:\n• Modern professional design\n• Responsive layout\n• User-friendly navigation\n• Mobile compatibility\n• Clean aesthetics"
        
        # Default
        else:
            return f"I'm Mohanlal, your cybersecurity mentor. Ask me about:\n\n🚀 'project' - Platform overview\n🛍️ 'vulnshop' - Practice environment\n✨ 'features' - Platform capabilities\n🛣️ 'roadmap' - Learning path\n🔍 'reconnaissance' - Information gathering\n🛠️ 'tools' - Hacking tools\n🎯 'penetration testing' - Methodology\n\nOr ask specific cybersecurity questions!"
        if security_response:
            return f"{random.choice(self.personality_responses)}{security_response}"
        
        return None
    
    def _get_topic_response(self, user_input):
        for category, topics in self.vulnerability_database.items():
            for topic, explanation in topics.items():
                if any(keyword in user_input for keyword in topic.split('_')):
                    return f"{explanation}\n\n🎯 Try this in our labs!"
        
        # Specific vulnerability responses
        if 'xss' in user_input:
            return "XSS Types: Reflected (<script>alert(1)</script>), Stored (<img src=x onerror=alert(1)>), DOM-based. Want exploitation examples?"
        elif 'sql' in user_input:
            return "SQL Injection: UNION (' UNION SELECT 1,2,3--), Boolean Blind (' AND 1=1--), Time-based (' AND SLEEP(5)--). Need payloads?"
        elif 'csrf' in user_input:
            return "CSRF Attacks: GET (<img src=malicious-url>), POST (auto-submit forms), JSON (text/plain bypass). Want examples?"
        
        return None
    
    def _answer_what_question(self, user_input):
        if 'xss' in user_input:
            return "XSS (Cross-Site Scripting) allows attackers to inject malicious scripts into web pages. There are 3 types: Stored, Reflected, and DOM-based."
        elif 'sql injection' in user_input or 'sqli' in user_input:
            return "SQL Injection occurs when user input is inserted into SQL queries without sanitization, allowing database manipulation."
        elif 'csrf' in user_input:
            return "CSRF (Cross-Site Request Forgery) tricks users into performing unwanted actions on authenticated websites."
        elif 'phishing' in user_input:
            return "Phishing is a social engineering attack where attackers impersonate trusted entities to steal credentials or data."
        else:
            return f"You're asking about '{user_input.replace('what is', '').replace('what are', '').strip()}'. Could you specify which security concept you'd like to know about?"
    
    def _answer_how_question(self, user_input):
        if 'prevent' in user_input and 'xss' in user_input:
            return "To prevent XSS: 1) Validate and sanitize input 2) Encode output 3) Use Content Security Policy 4) Escape HTML characters"
        elif 'prevent' in user_input and 'sql' in user_input:
            return "To prevent SQL injection: 1) Use parameterized queries 2) Validate input 3) Use stored procedures 4) Apply least privilege"
        elif 'hack' in user_input:
            return "Ethical hacking involves: 1) Get proper authorization 2) Use reconnaissance tools 3) Identify vulnerabilities 4) Document findings 5) Report responsibly"
        else:
            return f"You're asking how to handle '{user_input.replace('how to', '').replace('how do', '').replace('how can', '').strip()}'. What specific security task do you need help with?"
    
    def _answer_why_question(self, user_input):
        if 'important' in user_input:
            return "Cybersecurity is crucial because: 1) Protects sensitive data 2) Prevents financial losses 3) Maintains business reputation 4) Ensures compliance"
        elif 'vulnerable' in user_input:
            return "Systems become vulnerable due to: 1) Unpatched software 2) Weak passwords 3) Poor coding practices 4) Misconfiguration"
        else:
            return "Security measures exist to protect against evolving cyber threats and maintain system integrity."
    
    def _answer_when_question(self, user_input):
        if 'test' in user_input:
            return "Security testing should be done: 1) During development 2) Before deployment 3) After updates 4) Regularly (monthly/quarterly)"
        elif 'update' in user_input:
            return "Update security measures: 1) Immediately for critical patches 2) Weekly for regular updates 3) After security incidents"
        else:
            return "Security is an ongoing process that requires continuous monitoring and regular updates."
    
    def _answer_where_question(self, user_input):
        if 'learn' in user_input:
            return "Learn cybersecurity from: 1) Our vulnerability labs 2) OWASP documentation 3) Security courses 4) Bug bounty platforms"
        elif 'find' in user_input:
            return "Find vulnerabilities in: 1) Web applications 2) Network services 3) Mobile apps 4) System configurations"
        else:
            return "Security vulnerabilities can be found in applications, networks, systems, and human processes."
    
    def _check_security_terms(self, user_input):
        security_terms = {
            'password': "Strong passwords should be 12+ characters with mixed case, numbers, and symbols. Use unique passwords for each account.",
            'firewall': "Firewalls filter network traffic based on security rules. They're your first line of defense against network attacks.",
            'encryption': "Encryption scrambles data so only authorized parties can read it. Always use HTTPS and encrypt sensitive data.",
            'malware': "Malware is malicious software designed to damage systems. Keep antivirus updated and avoid suspicious downloads.",
            'vulnerability': "A vulnerability is a weakness that can be exploited by attackers. Regular scanning and patching are essential.",
            'penetration testing': "Pen testing simulates real attacks to find vulnerabilities before malicious actors do.",
            'social engineering': "Social engineering manipulates people to reveal information. Always verify requests and be suspicious of urgency."
        }
        
        for term, explanation in security_terms.items():
            if term in user_input:
                return f"{explanation} Need more details about {term}?"
        
        return None
    
    def _get_lab_context(self, user_input):
        """Add relevant lab context to improve AI responses"""
        lab_contexts = {
            'xss': 'We have XSS labs including reflected XSS in search and stored XSS in comments. Users practice with <script> tags and event handlers.',
            'sql injection': 'Our SQL injection lab uses a vulnerable login form. Users learn about bypassing authentication with OR 1=1 and UNION attacks.',
            'csrf': 'CSRF lab demonstrates token bypass in profile updates. Users create malicious forms to change user data.',
            'xxe': 'XXE lab shows XML parsing vulnerabilities. Users inject external entities to read local files.',
            'idor': 'IDOR lab has predictable user IDs in URLs. Users access other profiles by changing ID parameters.',
            'lab': 'Our platform has 12 vulnerability labs covering OWASP Top 10. Each lab has hints, solutions, and flag submission.'
        }
        
        user_lower = user_input.lower()
        for term, context in lab_contexts.items():
            if term in user_lower:
                return context
        return None

# Initialize security mentor AI
security_ai = SecurityMentorAI()

def get_user_by_id(user_id):
    return User.query.get(user_id)

# User Model
class User(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    username = db.Column(db.String(80), unique=True, nullable=False)
    name = db.Column(db.String(100), nullable=False)
    email = db.Column(db.String(120), unique=True, nullable=False)
    password_hash = db.Column(db.String(128), nullable=False)
    score = db.Column(db.Integer, default=0)
    labs_completed = db.Column(db.Integer, default=0)
    is_verified = db.Column(db.Boolean, default=True)
    failed_attempts = db.Column(db.Integer, default=0)
    locked_until = db.Column(db.DateTime, nullable=True)
    birthdate = db.Column(db.Date, nullable=True)
    created_at = db.Column(db.DateTime, default=datetime.utcnow)

# Vulnerability Model
class Vulnerability(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    title = db.Column(db.String(200), nullable=False)
    description = db.Column(db.Text, nullable=False)
    impact = db.Column(db.Text, nullable=False)
    hint = db.Column(db.Text, nullable=False)
    solution = db.Column(db.Text, nullable=False)
    difficulty = db.Column(db.String(20), nullable=False)
    vuln_type = db.Column(db.String(50), nullable=False)
    flag = db.Column(db.String(100), nullable=False)
    created_at = db.Column(db.DateTime, default=datetime.utcnow)

class UserFlag(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    user_id = db.Column(db.Integer, db.ForeignKey('user.id'), nullable=False)
    vulnerability_id = db.Column(db.String(50), nullable=False)
    flag_submitted = db.Column(db.String(100), nullable=False)
    is_correct = db.Column(db.Boolean, default=False)
    submitted_at = db.Column(db.DateTime, default=datetime.utcnow)

@app.route('/')
def index():
    return render_template('index.html')

def validate_password(password):
    if len(password) < 8:
        return "Password must be at least 8 characters"
    if not re.search(r'[A-Z]', password):
        return "Password must contain uppercase letter"
    if not re.search(r'[a-z]', password):
        return "Password must contain lowercase letter"
    if not re.search(r'\d', password):
        return "Password must contain number"
    if not re.search(r'[!@#$%^&*(),.?":{}|<>]', password):
        return "Password must contain special character"
    return None

def check_rate_limit(ip, attempt_type, max_attempts=5, window_minutes=15):
    now = datetime.utcnow()
    attempts = register_attempts if attempt_type == 'register' else login_attempts
    
    # Clean old attempts
    attempts[ip] = [t for t in attempts[ip] if now - t < timedelta(minutes=window_minutes)]
    
    if len(attempts[ip]) >= max_attempts:
        return False
    
    attempts[ip].append(now)
    return True

@app.route('/register', methods=['POST'])
def register():
    ip = request.remote_addr
    
    if not check_rate_limit(ip, 'register'):
        return jsonify({'error': 'Too many registration attempts. Try again in 15 minutes'}), 429
    
    data = request.get_json()
    username = data.get('username', '').strip()
    name = data.get('name', '').strip()
    email = data.get('email', '').strip().lower()
    password = data.get('password', '')
    
    # Validation
    if not username or len(username) < 3:
        return jsonify({'error': 'Username must be at least 3 characters'}), 400
    
    if not re.match(r'^[a-zA-Z0-9_]+$', username):
        return jsonify({'error': 'Username can only contain letters, numbers, and underscores'}), 400
    
    if not re.match(r'^[^@]+@[^@]+\.[^@]+$', email):
        return jsonify({'error': 'Invalid email format'}), 400
    
    password_error = validate_password(password)
    if password_error:
        return jsonify({'error': password_error}), 400
    
    if User.query.filter_by(username=username).first():
        return jsonify({'error': 'Username already exists'}), 400
    
    if User.query.filter_by(email=email).first():
        return jsonify({'error': 'Email already exists'}), 400
    
    password_hash = generate_password_hash(password)
    user = User(username=username, name=name, email=email, password_hash=password_hash)
    
    db.session.add(user)
    db.session.commit()
    
    # Set session
    session['user_id'] = user.id
    
    access_token = create_access_token(identity=user.id)
    return jsonify({
        'access_token': access_token,
        'user': {
            'id': user.id,
            'username': user.username,
            'name': user.name,
            'email': user.email,
            'score': user.score,
            'labs_completed': user.labs_completed
        }
    }), 201

@app.route('/login', methods=['POST'])
def login():
    ip = request.remote_addr
    
    if not check_rate_limit(ip, 'login'):
        return jsonify({'error': 'Too many login attempts. Try again in 15 minutes'}), 429
    
    data = request.get_json()
    username = data.get('username', '').strip()
    password = data.get('password', '')
    
    if not username or not password:
        return jsonify({'error': 'Username and password required'}), 400
    
    user = User.query.filter_by(username=username).first()
    
    if not user:
        return jsonify({'error': 'Invalid credentials'}), 401
    
    # Check if account is locked
    if user.locked_until and datetime.utcnow() < user.locked_until:
        return jsonify({'error': 'Account temporarily locked. Try again later'}), 423
    
    if check_password_hash(user.password_hash, password):
        # Reset failed attempts on successful login
        user.failed_attempts = 0
        user.locked_until = None
        db.session.commit()
        
        # Set session
        session['user_id'] = user.id
        session['username'] = user.username
        session['email'] = user.email
        session['username'] = user.username
        session['email'] = user.email
        session['username'] = user.username
        session['email'] = user.email
        
        access_token = create_access_token(identity=user.id)
        return jsonify({
            'access_token': access_token,
            'user': {
                'id': user.id,
                'username': user.username,
                'name': user.name,
                'email': user.email,
                'score': user.score,
                'labs_completed': user.labs_completed
            }
        }), 200
    else:
        # Increment failed attempts
        user.failed_attempts += 1
        if user.failed_attempts >= 5:
            user.locked_until = datetime.utcnow() + timedelta(minutes=30)
        db.session.commit()
        
        return jsonify({'error': 'Invalid credentials'}), 401

@app.route('/api/leaderboard')
def api_leaderboard():
    leaderboard = User.query.order_by(User.score.desc()).limit(10).all()
    return jsonify([
        {
            'rank': idx + 1,
            'username': user.username,
            'score': user.score,
            'labs_completed': user.labs_completed
        }
        for idx, user in enumerate(leaderboard)
    ])

@app.route('/api/user-stats')
@jwt_required()
def api_user_stats():
    user_id = get_jwt_identity()
    user = User.query.get(user_id)
    
    if not user:
        return jsonify({'error': 'User not found'}), 404
    
    # Get recent submissions
    recent_submissions = UserFlag.query.filter_by(user_id=user_id).order_by(
        UserFlag.submitted_at.desc()
    ).limit(5).all()
    
    # Get user rank
    users_above = User.query.filter(User.score > user.score).count()
    user_rank = users_above + 1
    
    return jsonify({
        'username': user.username,
        'score': user.score,
        'labs_completed': user.labs_completed,
        'rank': user_rank,
        'recent_submissions': [
            {
                'lab_id': sub.vulnerability_id,
                'is_correct': sub.is_correct,
                'submitted_at': sub.submitted_at.isoformat()
            }
            for sub in recent_submissions
        ]
    })

@app.route('/api/lab-progress')
@jwt_required()
def api_lab_progress():
    user_id = get_jwt_identity()
    
    # Get all vulnerabilities with completion status
    vulnerabilities = Vulnerability.query.all()
    completed_labs = db.session.query(UserFlag.vulnerability_id).filter_by(
        user_id=user_id, is_correct=True
    ).distinct().all()
    completed_lab_ids = [lab[0] for lab in completed_labs]
    
    return jsonify([
        {
            'id': vuln.id,
            'title': vuln.title,
            'difficulty': vuln.difficulty,
            'completed': vuln.id in completed_lab_ids
        }
        for vuln in vulnerabilities
    ])



@app.route('/dashboard')
def dashboard():
    labs = [
        {'id': 1, 'title': 'Stored XSS', 'description': 'Exploit stored XSS in review system', 'difficulty': 'Beginner', 'points': 100, 'url': '/lab/stored_xss', 'flag_type': 'stored_xss'},
        {'id': 2, 'title': 'Reflected XSS', 'description': 'Find reflected XSS vulnerabilities', 'difficulty': 'Beginner', 'points': 100, 'url': '/lab/reflected_xss', 'flag_type': 'reflected_xss'},
        {'id': 3, 'title': 'SQL Injection', 'description': 'Bypass login using SQL injection', 'difficulty': 'Intermediate', 'points': 150, 'url': '/lab/sql_injection', 'flag_type': 'sql_injection'},
        {'id': 4, 'title': 'CSRF Attack', 'description': 'Exploit missing CSRF protection', 'difficulty': 'Intermediate', 'points': 120, 'url': '/lab/csrf_attack', 'flag_type': 'csrf_attack'},
        {'id': 5, 'title': 'XXE Injection', 'description': 'Exploit XXE in review XML processing', 'difficulty': 'Advanced', 'points': 200, 'url': '/lab/xxe_injection', 'flag_type': 'xxe_injection'},
        {'id': 6, 'title': 'LFI Attack', 'description': 'Access local files through LFI', 'difficulty': 'Intermediate', 'points': 130, 'url': '/lab/lfi_attack', 'flag_type': 'lfi_attack'},
        {'id': 7, 'title': 'RCE Exploit', 'description': 'Execute remote code on server', 'difficulty': 'Advanced', 'points': 250, 'url': '/lab/rce_exploit', 'flag_type': 'rce_exploit'},
        {'id': 8, 'title': 'IDOR Bypass', 'description': 'Access unauthorized resources via IDOR', 'difficulty': 'Intermediate', 'points': 140, 'url': '/lab/idor_bypass', 'flag_type': 'idor_bypass'},
        {'id': 9, 'title': 'Auth Bypass', 'description': 'Bypass authentication mechanisms', 'difficulty': 'Advanced', 'points': 180, 'url': '/lab/auth_bypass', 'flag_type': 'auth_bypass'},
        {'id': 10, 'title': 'Session Hijack', 'description': 'Hijack user sessions', 'difficulty': 'Intermediate', 'points': 160, 'url': '/lab/session_hijack', 'flag_type': 'session_hijack'},
        {'id': 11, 'title': 'File Upload', 'description': 'Bypass file upload restrictions', 'difficulty': 'Advanced', 'points': 170, 'url': '/lab/file_upload', 'flag_type': 'file_upload'},
        {'id': 12, 'title': 'Open Redirect', 'description': 'Exploit open redirect vulnerabilities', 'difficulty': 'Beginner', 'points': 90, 'url': '/lab/open_redirect', 'flag_type': 'open_redirect'}
    ]
    
    # Get current user and leaderboard data
    current_user = None
    leaderboard = []
    completed_labs = []
    
    try:
        verify_jwt_in_request(optional=True)
        user_id = get_jwt_identity()
        if user_id:
            current_user = User.query.get(user_id)
            # Get completed labs for this user
            completed_flags = UserFlag.query.filter_by(user_id=user_id, is_correct=True).all()
            completed_labs = [flag.vulnerability_id for flag in completed_flags]
    except:
        pass
    
    # Get leaderboard
    leaderboard = User.query.order_by(User.score.desc()).limit(10).all()
    
    return render_template('dashboard.html', 
                         vulnerabilities=labs, 
                         current_user=current_user, 
                         leaderboard=leaderboard,
                         completed_labs=completed_labs)

@app.route('/profile')
def profile():
    # Check if user is logged in via JWT or session
    user = None
    try:
        # First try JWT
        verify_jwt_in_request(optional=True)
        user_id = get_jwt_identity()
        if user_id:
            user = User.query.get(user_id)
        # If no JWT, try session
        elif 'user_id' in session:
            user = User.query.get(session['user_id'])
    except:
        # Fallback to session if JWT fails
        if 'user_id' in session:
            user = User.query.get(session['user_id'])
    
    return render_template('profile.html', user=user, submissions=[])

@app.route('/recon')
def recon():
    return render_template('recon.html')

@app.route('/update-profile', methods=['POST'])
@jwt_required()
def update_profile():
    user_id = get_jwt_identity()
    user = User.query.get(user_id)
    
    if not user:
        return jsonify({'error': 'User not found'}), 404
    
    data = request.get_json()
    
    # Update email if provided and not taken
    if 'email' in data and data['email'] != user.email:
        if User.query.filter_by(email=data['email']).first():
            return jsonify({'error': 'Email already exists'}), 400
        user.email = data['email']
    
    # Update birthdate if provided
    if 'birthdate' in data and data['birthdate']:
        try:
            user.birthdate = datetime.strptime(data['birthdate'], '%Y-%m-%d').date()
        except ValueError:
            return jsonify({'error': 'Invalid date format'}), 400
    
    db.session.commit()
    return jsonify({'message': 'Profile updated successfully'})

@app.route('/change-password', methods=['POST'])
@jwt_required()
def change_password():
    user_id = get_jwt_identity()
    user = User.query.get(user_id)
    
    if not user:
        return jsonify({'error': 'User not found'}), 404
    
    data = request.get_json()
    current_password = data.get('current_password')
    new_password = data.get('new_password')
    
    if not check_password_hash(user.password_hash, current_password):
        return jsonify({'error': 'Current password is incorrect'}), 400
    
    password_error = validate_password(new_password)
    if password_error:
        return jsonify({'error': password_error}), 400
    
    user.password_hash = generate_password_hash(new_password)
    db.session.commit()
    
    return jsonify({'message': 'Password changed successfully'})

@app.route('/api/ai-hint/<int:lab_id>')
@jwt_required()
def get_ai_hint(lab_id):
    vulnerability = Vulnerability.query.get_or_404(lab_id)
    return jsonify({'hint': vulnerability.hint})

@app.route('/api/ai-explanation/<int:lab_id>')
@jwt_required()
def get_ai_explanation(lab_id):
    vulnerability = Vulnerability.query.get_or_404(lab_id)
    return jsonify({'explanation': vulnerability.solution})

@app.route('/api/ai-chat', methods=['POST'])
def ai_chat():
    data = request.get_json()
    user_message = data.get('message', '').strip()
    
    # Use advanced security mentor AI
    try:
        ai_response = security_ai.generate_response(user_message)
        return jsonify({'response': ai_response})
    except Exception as e:
        return jsonify({'response': '🚀 Oops! My circuits got tangled. Try asking about web security, ethical hacking, cloud security, or mobile security!'})
    


@app.route('/api/user-profile')
@jwt_required()
def get_user_profile():
    user_id = get_jwt_identity()
    user = User.query.get(user_id)
    if not user:
        return jsonify({'error': 'User not found'}), 404
    
    return jsonify({
        'id': user.id,
        'username': user.username,
        'name': user.name,
        'email': user.email,
        'score': user.score,
        'labs_completed': user.labs_completed,
        'created_at': user.created_at.strftime('%B %d, %Y')
    })

@app.route('/lab/<int:lab_id>')
def lab_detail(lab_id):
    lab_details = {
        1: {
            'title': 'Stored Cross-Site Scripting (XSS)',
            'description': 'Learn to exploit stored XSS vulnerabilities in web applications',
            'objective': 'Inject malicious JavaScript code that gets stored and executed for other users',
            'instructions': [
                'Navigate to the review section of VulnShop',
                'Submit a review containing XSS payload like <script>alert(1)</script>',
                'Try different XSS vectors: <img src=x onerror=alert(1)>',
                'Observe how the payload executes when the page loads'
            ],
            'hints': [
                'Look for input fields that display user content',
                'Try HTML event handlers like onerror, onload',
                'Check if input is sanitized or encoded'
            ]
        },
        2: {
            'title': 'Reflected Cross-Site Scripting (XSS)',
            'description': 'Discover reflected XSS vulnerabilities in search functionality',
            'objective': 'Execute JavaScript code through URL parameters or form inputs',
            'instructions': [
                'Use the search functionality in VulnShop',
                'Try searching for <script>alert(1)</script>',
                'Test different XSS payloads in the search box',
                'Check if the search term is reflected in the page'
            ],
            'hints': [
                'Search functionality often reflects user input',
                'Try javascript: protocol in URLs',
                'Look for unencoded output in HTML'
            ]
        },
        3: {
            'title': 'SQL Injection',
            'description': 'Exploit SQL injection vulnerabilities to bypass authentication',
            'objective': 'Manipulate SQL queries to gain unauthorized access',
            'instructions': [
                'Go to the login form in VulnShop',
                'Try SQL injection payloads like admin\'--',
                'Test with \' OR \'1\'=\'1\' -- in username field',
                'Observe if you can bypass authentication'
            ],
            'hints': [
                'Single quotes often break SQL queries',
                'Comment out rest of query with --',
                'Try UNION SELECT statements'
            ]
        },
        4: {
            'title': 'Cross-Site Request Forgery (CSRF)',
            'description': 'Exploit missing CSRF protection in forms',
            'objective': 'Force users to perform unintended actions',
            'instructions': [
                'Look for forms without CSRF tokens',
                'Try submitting forms from external pages',
                'Check if requests can be forged',
                'Test profile update functionality'
            ],
            'hints': [
                'Look for missing anti-CSRF tokens',
                'Check if requests validate origin',
                'Forms should have hidden token fields'
            ]
        },
        5: {
            'title': 'XML External Entity (XXE) Injection',
            'description': 'Exploit XXE vulnerabilities in XML processing',
            'objective': 'Access local files through XML external entities',
            'instructions': [
                'Find XML input fields in reviews',
                'Try XXE payload: <!DOCTYPE foo [<!ENTITY xxe SYSTEM "file:///etc/passwd">]>',
                'Submit XML with external entity references',
                'Check if file contents are disclosed'
            ],
            'hints': [
                'Look for XML processing functionality',
                'Try accessing /etc/passwd or C:\\Windows\\System32\\drivers\\etc\\hosts',
                'Use SYSTEM entities to read files'
            ]
        },
        6: {
            'title': 'Local File Inclusion (LFI)',
            'description': 'Access local files through path traversal',
            'objective': 'Read sensitive files from the server filesystem',
            'instructions': [
                'Look for file parameter in URLs',
                'Try path traversal: ../ sequences',
                'Attempt to access /etc/passwd',
                'Test different file paths'
            ],
            'hints': [
                'Use ../ to traverse directories',
                'Try accessing system files',
                'Look for file inclusion parameters'
            ]
        },
        7: {
            'title': 'Remote Code Execution (RCE)',
            'description': 'Execute arbitrary commands on the server',
            'objective': 'Gain command execution through vulnerable inputs',
            'instructions': [
                'Find command injection points',
                'Try command separators: ; | &&',
                'Test with system commands like whoami',
                'Look for file upload or processing features'
            ],
            'hints': [
                'Command separators can chain commands',
                'Try system(), exec(), eval() functions',
                'Look for file processing functionality'
            ]
        },
        8: {
            'title': 'Insecure Direct Object Reference (IDOR)',
            'description': 'Access unauthorized resources by manipulating object references',
            'objective': 'Access other users\' data by changing ID parameters',
            'instructions': [
                'Look for ID parameters in URLs',
                'Try changing user IDs or object IDs',
                'Access profile pages with different IDs',
                'Check if authorization is properly implemented'
            ],
            'hints': [
                'Sequential IDs are often vulnerable',
                'Try incrementing/decrementing ID values',
                'Look for user=, id=, profile= parameters'
            ]
        },
        9: {
            'title': 'Authentication Bypass',
            'description': 'Bypass authentication mechanisms',
            'objective': 'Gain access without valid credentials',
            'instructions': [
                'Try default credentials (admin/admin)',
                'Look for authentication bypass techniques',
                'Test parameter manipulation',
                'Check for weak password policies'
            ],
            'hints': [
                'Try common default passwords',
                'Look for admin panels',
                'Check for role-based access controls'
            ]
        },
        10: {
            'title': 'Session Hijacking',
            'description': 'Exploit weak session management',
            'objective': 'Hijack or manipulate user sessions',
            'instructions': [
                'Analyze session tokens',
                'Check for predictable session IDs',
                'Test session fixation attacks',
                'Look for session management flaws'
            ],
            'hints': [
                'Session tokens should be random',
                'Check if sessions expire properly',
                'Look for session cookies'
            ]
        },
        11: {
            'title': 'File Upload Bypass',
            'description': 'Bypass file upload restrictions',
            'objective': 'Upload malicious files to the server',
            'instructions': [
                'Find file upload functionality',
                'Try uploading different file types',
                'Test with .php, .jsp, .exe files',
                'Use double extensions like file.php.jpg'
            ],
            'hints': [
                'Try different file extensions',
                'Look for MIME type validation',
                'Test file size restrictions'
            ]
        },
        12: {
            'title': 'Open Redirect',
            'description': 'Exploit open redirect vulnerabilities',
            'objective': 'Redirect users to malicious websites',
            'instructions': [
                'Look for redirect parameters in URLs',
                'Try url=, redirect=, goto= parameters',
                'Test with external domains',
                'Check if validation is performed'
            ],
            'hints': [
                'Look for redirect functionality',
                'Try different URL schemes',
                'Check for domain validation'
            ]
        }
    }
    
    lab_info = lab_details.get(lab_id, {
        'title': f'Lab {lab_id}',
        'description': 'Vulnerability testing lab',
        'objective': 'Find and exploit vulnerabilities',
        'instructions': ['Launch the lab and start testing'],
        'hints': ['Use the AI assistant for help']
    })
    
    return render_template('lab_detail.html', lab_id=lab_id, lab_info=lab_info)

@app.route('/deploy-lab/<int:lab_id>')
def deploy_lab(lab_id):
    return render_template('vulnerable_site.html', lab_id=lab_id)



# Flag system - 12 vulnerabilities
flags = {
    'stored_xss': 'FLAG{STORED_XSS_MASTER_2024}',
    'reflected_xss': 'FLAG{REFLECTED_XSS_FOUND}',
    'sql_injection': 'FLAG{SQL_INJECTION_BYPASSED}',
    'csrf_attack': 'FLAG{CSRF_TOKEN_MISSING}',
    'xxe_injection': 'FLAG{XXE_PAYLOAD_SUCCESS}',
    'lfi_attack': 'FLAG{LOCAL_FILE_INCLUSION}',
    'rce_exploit': 'FLAG{REMOTE_CODE_EXECUTION}',
    'idor_bypass': 'FLAG{INSECURE_DIRECT_OBJECT_REF}',
    'auth_bypass': 'FLAG{BROKEN_AUTHENTICATION}',
    'session_hijack': 'FLAG{SESSION_HIJACKED}',
    'file_upload': 'FLAG{FILE_UPLOAD_BYPASS}',
    'open_redirect': 'FLAG{OPEN_REDIRECT_FOUND}'
}

@app.route('/submit_flag', methods=['POST'])
@jwt_required()
def submit_flag():
    try:
        data = request.get_json()
        submitted_flag = data.get('flag', '').strip()
        lab_type = data.get('lab_type', '').strip()
        
        print(f"Flag submission: {lab_type} = {submitted_flag}")
        
        current_user_id = get_jwt_identity()
        print(f"User ID: {current_user_id}")
        
        if not current_user_id:
            return jsonify({
                'success': False, 
                'message': 'Authentication required'
            }), 401
        
        # Check if flag is correct
        if lab_type in flags and submitted_flag == flags[lab_type]:
            print(f"Flag is correct for {lab_type}")
            
            # Check if user already submitted this flag
            existing_submission = UserFlag.query.filter_by(
                user_id=current_user_id, 
                vulnerability_id=lab_type,
                is_correct=True
            ).first()
            
            if existing_submission:
                print(f"Flag already submitted by user {current_user_id}")
                return jsonify({
                    'success': False, 
                    'message': 'Flag already submitted for this lab!'
                }), 400
            
            # Get user and update score
            user = User.query.get(current_user_id)
            if not user:
                return jsonify({
                    'success': False, 
                    'message': 'User not found'
                }), 404
            
            print(f"Current user score: {user.score}, labs: {user.labs_completed}")
            
            # Update user score and labs completed
            user.score = (user.score or 0) + 100
            user.labs_completed = (user.labs_completed or 0) + 1
            
            # Add new flag submission
            new_submission = UserFlag(
                user_id=current_user_id,
                vulnerability_id=lab_type,
                flag_submitted=submitted_flag,
                is_correct=True
            )
            
            db.session.add(new_submission)
            db.session.commit()
            
            print(f"Updated user score: {user.score}, labs: {user.labs_completed}")
            
            return jsonify({
                'success': True, 
                'message': f'Correct flag! You earned 100 points. Total score: {user.score}',
                'points': 100,
                'total_score': user.score,
                'labs_completed': user.labs_completed
            })
        else:
            print(f"Incorrect flag: {submitted_flag} for {lab_type}")
            print(f"Expected: {flags.get(lab_type)}")
            return jsonify({
                'success': False, 
                'message': 'Incorrect flag. Try again!'
            }), 400
            
    except Exception as e:
        print(f"Error in submit_flag: {str(e)}")
        db.session.rollback()
        return jsonify({
            'success': False, 
            'message': f'Error submitting flag: {str(e)}'
        }), 500

# Vulnerability detection endpoints
@app.route('/check_vulnerability', methods=['POST'])
def check_vulnerability():
    try:
        data = request.get_json()
        vuln_type = data.get('type')
        payload = data.get('payload', '')
        
        vulnerabilities_found = []
        
        if vuln_type == 'stored_xss':
            if '<script>' in payload.lower() or 'alert(' in payload.lower() or '<img src=x onerror=' in payload.lower():
                vulnerabilities_found.append({
                    'type': 'Stored XSS',
                    'description': 'Stored Cross-Site Scripting vulnerability detected in review system',
                    'flag': flags.get('stored_xss')
                })
        
        elif vuln_type == 'reflected_xss':
            if '<script>' in payload.lower() or 'javascript:' in payload.lower():
                vulnerabilities_found.append({
                    'type': 'Reflected XSS',
                    'description': 'Reflected Cross-Site Scripting vulnerability detected',
                    'flag': flags.get('reflected_xss')
                })
        
        elif vuln_type == 'sql_injection':
            sql_patterns = ["' or '1'='1", "' or 1=1--", "union select", "admin'--"]
            if any(pattern in payload.lower() for pattern in sql_patterns):
                vulnerabilities_found.append({
                    'type': 'SQL Injection',
                    'description': 'SQL Injection vulnerability detected in login form',
                    'flag': flags.get('sql_injection')
                })
        
        elif vuln_type == 'xxe_injection':
            if '<!entity' in payload.lower() or '&xxe;' in payload.lower() or 'file://' in payload.lower():
                vulnerabilities_found.append({
                    'type': 'XXE Injection',
                    'description': 'XML External Entity vulnerability detected in review system',
                    'flag': flags.get('xxe_injection')
                })
        
        elif vuln_type == 'csrf_attack':
            if 'csrf' in payload.lower() or 'token' in payload.lower():
                vulnerabilities_found.append({
                    'type': 'CSRF Attack',
                    'description': 'Cross-Site Request Forgery vulnerability detected',
                    'flag': flags.get('csrf_attack')
                })
        
        elif vuln_type == 'lfi_attack':
            if '../' in payload or '/etc/passwd' in payload.lower():
                vulnerabilities_found.append({
                    'type': 'LFI Attack',
                    'description': 'Local File Inclusion vulnerability detected',
                    'flag': flags.get('lfi_attack')
                })
        
        elif vuln_type == 'rce_exploit':
            if ';' in payload or '|' in payload or '&&' in payload:
                vulnerabilities_found.append({
                    'type': 'RCE Exploit',
                    'description': 'Remote Code Execution vulnerability detected',
                    'flag': flags.get('rce_exploit')
                })
        
        elif vuln_type == 'idor_bypass':
            if 'id=' in payload.lower() or 'user=' in payload.lower():
                vulnerabilities_found.append({
                    'type': 'IDOR Bypass',
                    'description': 'Insecure Direct Object Reference vulnerability detected',
                    'flag': flags.get('idor_bypass')
                })
        
        elif vuln_type == 'auth_bypass':
            if 'admin' in payload.lower() or 'bypass' in payload.lower():
                vulnerabilities_found.append({
                    'type': 'Auth Bypass',
                    'description': 'Authentication bypass vulnerability detected',
                    'flag': flags.get('auth_bypass')
                })
        
        elif vuln_type == 'session_hijack':
            if 'session' in payload.lower() or 'cookie' in payload.lower():
                vulnerabilities_found.append({
                    'type': 'Session Hijack',
                    'description': 'Session hijacking vulnerability detected',
                    'flag': flags.get('session_hijack')
                })
        
        elif vuln_type == 'file_upload':
            if '.php' in payload.lower() or '.jsp' in payload.lower() or 'shell' in payload.lower():
                vulnerabilities_found.append({
                    'type': 'File Upload',
                    'description': 'File upload bypass vulnerability detected',
                    'flag': flags.get('file_upload')
                })
        
        elif vuln_type == 'open_redirect':
            if 'redirect' in payload.lower() or 'url=' in payload.lower():
                vulnerabilities_found.append({
                    'type': 'Open Redirect',
                    'description': 'Open redirect vulnerability detected',
                    'flag': flags.get('open_redirect')
                })
        
        return jsonify({
            'vulnerabilities': vulnerabilities_found,
            'total_found': len(vulnerabilities_found)
        })
        
    except Exception as e:
        return jsonify({'error': str(e)}), 500

if __name__ == '__main__':
    with app.app_context():
        # Create database tables
        print("Creating database tables...")
        db.create_all()
        print("Database tables created successfully")
    
    app.run(host='127.0.0.1', port=5000, debug=True)


