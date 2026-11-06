# Philippine Bar Recitation Simulator Suite: Master Technical Evolution & Changelog

This document chronicles the complete development history, architectural decisions, bug fixes, storage migrations, and feature milestones of the Philippine Bar Recitation Simulator suite from its initial multi-variant inception through its evolution into a cross-platform Tauri application with serverless 6-digit PIN pairing sync, session auto-resume, and advanced deck regeneration capabilities.

---

## 1. Core Engine & AI Interrogation Architecture

* **Dual-Option & Multi-Variant Architecture (Options 2a, 2b, 3a, 3b):**
  * *Codal & Statutory Variants (2a/3a):* Tailored AI interrogation prompts, extraction parsers, and evaluation rubrics targeting statutory provisions, Rules/Articles/Sections, essential elements/requisites, and express exceptions.
  * *Case FIR Variants (2b/3b):* Re-engineered professor mandates, agenda extraction, and grading schemas to execute formal Supreme Court oral case recitations using the Facts-Issue-Ruling (FIR) methodology.

* **Turn-by-Turn On-Demand Generation Architecture:**
  * Replaced legacy multi-question batch generation models with a two-phase pipeline:
    1. *Boot/Setup Phase:* Lightweight agenda extraction (`extractStructuredAgenda`) parses syllabus files or scope constraints into clean arrays of titles/citations (~200 tokens), preventing output cap failures on boot.
    2. *Per-Turn Phase:* Questions and case calls are generated individually on demand (~120–150 tokens) strictly rotating through structured typologies.

* **Decoupled Single-Task Evaluation (ALAC & FIR):**
  * Purged next-question/next-case payload generation from evaluation requests (`submitStudentAnswer`). Gemini grades student submissions exclusively under substantial compliance standards, halving output token consumption per turn and eliminating rate-limit spikes.

* **Asymmetric Jurisprudential & Statutory Token Excerpt Sizing:**
  * Optimized token payloads by separating task-specific retrieval budgets:
    * *Question/Case Call Generation:* Capped at $4,000$ characters for fast 1.5–2.5 second turnaround.
    * *Diagnostic Evaluation / Grading:* Capped at $8,000$ to $10,000$ characters, anchored by priority section/case-title weighting multipliers ($\times 50$).

* **Recitation-Filter Mitigation (`temperature: 0.1`) & Explicit Safety Overrides (`BLOCK_NONE`):**
  * Configured generation temperature to $0.1$ and structured benchmark schemas into operative rule summaries, essential elements, and exceptions to bypass Google's automated pre-training memorization `RECITATION` filters.
  * Configured explicit `safetySettings` with `BLOCK_NONE` across all harmful content categories (`HARASSMENT`, `HATE_SPEECH`, `SEXUALLY_EXPLICIT`, `DANGEROUS_CONTENT`) to prevent true-to-life legal fact patterns involving crimes or injuries from triggering false-positive AI blocks.

* **Multi-Key & Multi-Model Failover Cascade:**
  * Integrated automated API key hot-swapping across comma-separated multi-project keys (`AIzaSyKey1..., AIzaSyKey2...`) and a robust finite linear model cascade (`gemini-3.8-flash` $\rightarrow$ `gemini-3.6-flash` $\rightarrow$ `gemini-3.7-flash` $\rightarrow$ `gemini-3.5-flash`) with 2.5-second pacing delays on HTTP 429/503 responses and a 40-second watchdog timer.

---

## 2. UI/UX Interrogation Interface & Modals

* **Spoiler-Free Question Cards:**
  * Hidden active topic badges and typology headers from live question screens. Questions display strictly as **`Question X of Y`**, forcing students to perform pure issue-spotting without premature hints. Contextual topic disclosure is restricted to diagnostic evaluations and debrief screens.

* **Polymorphic Diagnostic Evaluation & Debrief Modals:**
  * Standardized setup, diagnostic evaluation, transcript, and debrief modals across all variants, enabling users to review professor critiques, missed statutory elements, and authoritative benchmark breakdowns. Filter options include *All* vs. *Deficient Only*.

* **Rich Markdown Study Sheet Exporter (`.md`) & Word Ready (`.doc`):**
  * Integrated study sheet export utilities formatting transcripts into numbered bold questions followed by clean structured paragraphs for seamless integration into Obsidian, Notion, or Microsoft Word with proper UTF-8 BOM encoding.

* **Custom Flashcard Studio, Bulk Ingestion & Cloze Deletions:**
  * Provided authoring tools supporting 5 distinct legal archetypes (Codal, Case FIR, Bar ALAC, Procedural, and Simple Q&A) with RemNote-style Cloze Deletions (`{{c1::...}}` with optional hints), bulk CSV/TSV/Anki import, and live in-place card editing (`Ctrl+E` or `E`) without losing active session state.

---

## 3. Storage & Cross-Device Synchronization

* **Isolated Subject-Specific IndexedDB Databases:**
  * Implemented dynamic database sharding tied to the active subject workspace and version (e.g., `BarRecitUniversalMasterDB_v14_1`), ensuring complete isolation of mastery ledgers, syllabi, and reference sources across different law subjects without cross-contamination.

* **Adaptive Mastery Ledger & Spaced Repetition:**
  * Maintained local tracking of student performance (`EXCELLENT`, `SATISFACTORY`, `DEFICIENT`), categorizing provisions and cases into *Critical Drill*, *Practicing*, and *Mastered* ledgers stored inside IndexedDB with automated weak-spot resurfacing and priority re-queuing (Leitner queue).

* **Serverless 6-Digit Device Pairing Sync:**
  * Replaced browser-blocked Google OAuth AppData flows with an ephemeral, zero-login 6-digit device pairing snapshot relay (`KVdb.io`).
  * *Workflow:* The primary device packages decks, sources, and mastery into a compressed JSON snapshot tied to a temporary 6-digit PIN (valid for 15 minutes). Entering this PIN on a secondary device (Android APK, tablet, or laptop) pulls and non-destructively merges all state in under 2 seconds.

* **Local Native Folder Backup & Master JSON Migration:**
  * Configured automated JSON subject auto-saving to local disk (`Documents\BarRecitationSync\`) for background synchronization via OneDrive, Google Drive Desktop, or Syncthing wrappers.
  * Maintained master workspace `.json` export and import utilities for manual cross-device migration.

---

## 4. Advanced Engine Enhancements & New Additions (Parts 4 & 5)

* **Session Auto-Resume (Part 4):**
  * Implemented automatic session state saving to `localStorage` (`bar_active_session_[Subject]`) on every card advance.
  * Added a startup check on app boot across all platforms (Web, macOS DMG, Android APK, and Windows EXE) that detects unfinished recitation runs and prompts: *"Found an active recitation session... Would you like to resume where you left off?"* Automatically clears when the batch concludes.

* **Multi-Paragraph & Sub-Clause Scope Parser Upgrade (Part 5A):**
  * Enhanced `expandDeterministicScopeItems` to automatically scan statutory sections for internal sub-letters `(a), (b), (c)` and multi-paragraph structures (such as Article 315 Estafa paragraphs 1, 2, and 3), unbundling them into distinct atomic cards rather than skipping or lumping them together into monolithic blocks.

* **Smart "Regenerate & Polish Deck" Tool (Part 5B):**
  * Added a **"✨ Polish"** button to every saved deck in the Library.
  * Built an interactive configuration modal displaying the deck's original settings (e.g., active dimensions and presets), allowing users to keep original configurations or modify presets (*Codal*, *Case FIR*, *Reviewer*, *Full Bar Sim*), strip low-value boilerplate (effectivity clauses), and inject active-recall Cloze Deletions while preserving Leitner mastery scores.

---

## 5. Cross-Platform Deployment & Tauri Distribution

* **Standalone Portability & Remote Webview Updates:**
  * Configured all simulators as completely self-contained single-file HTML applications incorporating Tailwind CSS and PDF.js via secure CDNs, requiring zero backend server installation or cloud configuration.
  * Maintained remote webview routing via GitHub Pages (`https://ph-bar-recit.github.io/bar-recitation-simulator/`) and Tauri GitHub Actions, ensuring that every `git push origin main` instantly propagates updates to all installed Tauri desktop and mobile client wrappers without requiring re-installations.