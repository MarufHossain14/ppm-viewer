## ✅ Step 1: Install `wasm-pack` (Windows)

### Option A: **Install via Cargo (Preferred)**

Open **Command Prompt** or **PowerShell** and run:

```bash
cargo install wasm-pack
```

This will download and build it locally.

> ⚠️ This may take a few minutes the first time. Be patient.

---

## ✅ Step 2: Add it to PATH (if needed)

After installation, `wasm-pack` should be in your Cargo bin directory. If the error persists:

1. Open Command Prompt or PowerShell.
2. Run:

```bash
echo %USERPROFILE%\.cargo\bin
```

Copy that output — it’s likely something like:

```
C:\Users\YourName\.cargo\bin
```

3. Add it to your **System PATH**:

   * Open **Start > "Edit the system environment variables"**
   * Click **Environment Variables**
   * Under **"User variables"**, find or add `Path`
   * Click **Edit**, then **New**, and paste in:
     `C:\Users\YourName\.cargo\bin`
   * Click OK until all windows close.

---

## ✅ Step 3: Confirm it's working

Close your terminal and open a **new** one. Then try:

```bash
wasm-pack --version
```

You should see something like:

```
wasm-pack 0.12.1
```

Now you can run:

```bash
wasm-pack build --target web
```