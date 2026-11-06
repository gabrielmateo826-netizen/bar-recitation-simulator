# Philippine Bar Recitation Simulator (Universal Hybrid Engine v14.0+)

The **Philippine Bar Recitation Simulator** is a hybrid, multi-variant legal interrogation and active-recall suite designed to simulate high-pressure Socratic cold-calls under strict Philippine Bar examination standards. Built as a self-contained hybrid web and desktop application, it operates with zero traditional backend server dependencies while supporting cross-platform deployment via GitHub Pages and Tauri (Windows EXE, macOS DMG, and Android APK).

---

## 🚀 Key Features

* **Universal Hybrid Engine:** Combine a course syllabus (Anchor 1) with reference case digests, codals, and reviewers (Anchor 2) paired with a typed coverage scope to generate verified, grounded recitation cards.
* **10-Dimension Recitation Matrix:** Tailor drills across specific interrogation styles:
  1. *Legal Definition & Nature*
  2. *Essential Elements & Requisites (Numbered)*
  3. *Statutory Exceptions & Provisos*
  4. *Doctrinal Distinctions*
  5. *Case Facts & Procedural Posture*
  6. *Case Holding & Ratio Decidendi*
  7. *Complete Case Brief (Full FIR)*
  8. *Comparative Precedent Synthesis (Case X vs. Y)*
  9. *Bar Problem Hypotheticals (4-Part ALAC)*
  10. *Procedural Remedies & Defenses*
* **Serverless 6-Digit Device Pairing Sync:** Wirelessly transfer decks, syllabus modules, and mastery progress between your PC, tablet, and phone instantly using an ephemeral 6-digit PIN relay (zero accounts, passwords, or Google OAuth WebView blocks required).
* **Adaptive Mastery Ledger & Leitner Spaced Repetition:** IndexedDB-backed weak-spot tracking that categorizes provisions and cases into *Critical Drill*, *Practicing*, and *Mastered* ledgers, automatically resurfacing weak points.
* **Multi-Project Gemini API Failover Cascade:** Automated API key hot-swapping across comma-separated multi-project keys with a robust model cascade (`gemini-3.8-flash` $\rightarrow$ `gemini-3.6-flash` $\rightarrow$ `gemini-3.5-flash`) and rate-limit auto-cooling.
* **Session Auto-Resume & Smart Deck Regeneration:** Automatically preserves your in-progress card index across app restarts (Web, DMG, APK, EXE) and provides AI-driven deck polishing with active-recall Cloze Deletions (`{{c1::...}}`).
* **Professional Study Sheet Exports:** Export recitation decks and transcripts as formatted Microsoft Word (`.doc`), Obsidian Markdown (`.md`), or JSON backup files.

---

## 🛠️ Quick Start & API Key Setup

1. **Get a Free Gemini API Key:**
   * Go to [Google AI Studio](https://aistudio.google.com/app/apikey) and sign in using a personal **`@gmail.com`** account (school/university domains restrict Google Cloud key creation).
   * Click **Create API key** in a new project.
   * *Tip:* Create 2 or 3 keys in separate projects and paste them comma-separated (`AIzaSyKey1..., AIzaSyKey2...`) to avoid free-tier rate limits during bulk compilation.

2. **Launch & Configure:**
   * Open the app in your browser or desktop wrapper.
   * Click **Scope & Decks** in the top navigation bar.
   * Paste your Gemini API key(s).
   * Attach your course syllabus or reference files, enter your target coverage scope, and click **Pre-Compile Full Deck** or **Launch Active Batch**.

---

## ☁️ Cross-Device 6-Digit PIN Sync

To sync your decks and mastery between your laptop and Android tablet/phone without Google account sign-in friction:
1. **On Device A (Primary):** Open Setup $\rightarrow$ Under *Cross-Device Instant Sync*, click **Generate 6-Digit PIN** (e.g., `482-195`).
2. **On Device B (Secondary):** Open the simulator $\rightarrow$ Type the 6-digit code into the pairing box and tap **Pull & Merge**. All decks and progress will merge locally in under 2 seconds.

---

## 📦 Cross-Platform Distribution (Tauri)

The simulator can be packaged as a native desktop or mobile application using Tauri:
* **Web:** Deployed instantly via GitHub Pages.
* **Desktop (Windows/Mac):** Compiled via Tauri into standalone `.exe` or `.dmg` binaries.
* **Mobile (Android):** Compiled into an `.apk` package for tablet and mobile recitation prep.

---

## 📄 License
Released under the MIT License. Designed for Philippine Bar candidates and law students.