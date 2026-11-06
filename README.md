# Philippine Bar Recitation Simulator (Universal Hybrid Engine v14.0+)

The **Philippine Bar Recitation Simulator** is a hybrid, multi-variant legal interrogation and active-recall suite designed to simulate high-pressure Socratic cold-calls under strict Philippine Bar examination standards[cite: 29]. Built as a self-contained hybrid web and desktop application, it operates with zero traditional backend server dependencies while supporting cross-platform deployment via GitHub Pages and Tauri (Windows EXE, macOS DMG, and Android APK)[cite: 29].

---

## 🌐 Instant Web Simulator & App Downloads

### 👉 [Launch Web Simulator (No Installation Required)](https://ph-bar-recit.github.io/bar-recitation-simulator/)

Prefer using it offline on your devices? Download the standalone native installers:

| Platform | File Format | Download Link |
| :--- | :--- | :--- |
| 🪟 **Windows** | `.exe` Installer | [📥 Download for Windows (EXE)](https://github.com/ph-bar-recit/bar-recitation-simulator/releases/latest) |
| 🍎 **macOS** | `.dmg` Package | [📥 Download for Mac (DMG)](https://github.com/ph-bar-recit/bar-recitation-simulator/releases/latest) |
| 📱 **Android** | `.apk` Package | [📥 Download for Android (APK)](https://github.com/ph-bar-recit/bar-recitation-simulator/releases/latest) |

*You can also find all released builds and historical updates under the repository's [Releases](https://github.com/ph-bar-recit/bar-recitation-simulator/releases) page.*

---

## 🚀 Key Features

* **Universal Hybrid Engine:** Combine a course syllabus (Anchor 1) with reference case digests, codals, and reviewers (Anchor 2) paired with a typed coverage scope to generate verified, grounded recitation cards[cite: 29].
* **10-Dimension Recitation Matrix:** Tailor drills across specific interrogation styles[cite: 29]:
  1. *Legal Definition & Nature*[cite: 29]
  2. *Essential Elements & Requisites (Numbered)*[cite: 29]
  3. *Statutory Exceptions & Provisos*[cite: 29]
  4. *Doctrinal Distinctions*[cite: 29]
  5. *Case Facts & Procedural Posture*[cite: 29]
  6. *Case Holding & Ratio Decidendi*[cite: 29]
  7. *Complete Case Brief (Full FIR)*[cite: 29]
  8. *Comparative Precedent Synthesis (Case X vs. Y)*[cite: 29]
  9. *Bar Problem Hypotheticals (4-Part ALAC)*[cite: 29]
  10. *Procedural Remedies & Defenses*[cite: 29]
* **Serverless 6-Digit Device Pairing Sync:** Wirelessly transfer decks, syllabus modules, and mastery progress between your PC, tablet, and phone instantly using an ephemeral 6-digit PIN relay (zero accounts, passwords, or Google OAuth WebView blocks required)[cite: 29].
* **Adaptive Mastery Ledger & Leitner Spaced Repetition:** IndexedDB-backed weak-spot tracking that categorizes provisions and cases into *Critical Drill*, *Practicing*, and *Mastered* ledgers, automatically resurfacing weak points[cite: 29].
* **Multi-Project Gemini API Failover Cascade:** Automated API key hot-swapping across comma-separated multi-project keys with a robust model cascade (`gemini-3.8-flash` $\rightarrow$ `gemini-3.6-flash` $\rightarrow$ `gemini-3.5-flash`) and rate-limit auto-cooling[cite: 29].
* **Session Auto-Resume & Smart Deck Regeneration:** Automatically preserves your in-progress card index across app restarts (Web, DMG, APK, EXE) and provides AI-driven deck polishing with adjustable card count sizing and active-recall Cloze Deletions (`{{c1::...}}`)[cite: 29].
* **Professional Study Sheet Exports:** Export recitation decks and transcripts as formatted Microsoft Word (`.doc`), Obsidian Markdown (`.md`), or JSON backup files[cite: 29].

---

## 🛠️ Quick Start & API Key Setup

1. **Get a Free Gemini API Key:**
   * Go to [Google AI Studio](https://aistudio.google.com/app/apikey) and sign in using a personal **`@gmail.com`** account (school/university domains restrict Google Cloud key creation)[cite: 29].
   * Click **Create API key** in a new project[cite: 29].
   * *Tip:* Create 2 or 3 keys in separate projects and paste them comma-separated (`AIzaSyKey1..., AIzaSyKey2...`) to avoid free-tier rate limits during bulk compilation[cite: 29].

2. **Launch & Configure:**
   * Open the app in your browser or desktop wrapper[cite: 29].
   * Click **Scope & Decks** in the top navigation bar[cite: 29].
   * Paste your Gemini API key(s)[cite: 29].
   * Attach your course syllabus or reference files, enter your target coverage scope, and click **Pre-Compile Full Deck** or **Launch Active Batch**[cite: 29].

---

## ☁️ Cross-Device 6-Digit PIN Sync

To sync your decks and mastery between your laptop and Android tablet/phone without Google account sign-in friction[cite: 29]:
1. **On Device A (Primary):** Open Setup $\rightarrow$ Under *Cross-Device Instant Sync*, click **Generate 6-Digit PIN** (e.g., `482-195`)[cite: 29].
2. **On Device B (Secondary):** Open the simulator $\rightarrow$ Type the 6-digit code into the pairing box and tap **Pull & Merge**[cite: 29]. All decks and progress will merge locally in under 2 seconds[cite: 29].

---

## 📦 Cross-Platform Distribution (Tauri)

The simulator can be packaged as a native desktop or mobile application using Tauri[cite: 29]:
* **Web:** Deployed instantly via GitHub Pages[cite: 29].
* **Desktop (Windows/Mac):** Compiled via Tauri into standalone `.exe` or `.dmg` binaries[cite: 29].
* **Mobile (Android):** Compiled into an `.apk` package for tablet and mobile recitation prep[cite: 29].

---

## 📄 License
Released under the MIT License[cite: 29]. Designed for Philippine Bar candidates and law students[cite: 29].