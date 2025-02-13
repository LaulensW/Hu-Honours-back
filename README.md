# 🚀 HU Honours - Frontend  

Dit is de frontend van de HU Honours leerplatform-app, gebouwd met **Expo (React Native)**. Deze app biedt een interactieve leerervaring, vergelijkbaar met Duolingo.  

## 📂 Projectstructuur  

Hier is een overzicht van de belangrijkste mappen en bestanden in deze repository en hun functies:  


# Branch Strategie  

Deze repository gebruikt een gestructureerde branching-strategie om samenwerking soepel te laten verlopen.  

---

## 🚀 Branch Overzicht  

| Branch         | Doel |
|---------------|------|
| `main`        | Stabiele productiecode. Alleen goedgekeurde code komt hier terecht. |
| `dev`         | Hoofdbranch voor actieve ontwikkeling. Nieuwe features en bugfixes worden hier samengevoegd. |
| `feature/*`   | Branches voor nieuwe functionaliteiten. Worden uiteindelijk in `dev` gemerged. |
| `bugfix/*`    | Branches voor bugfixes. Worden snel in `dev` en indien nodig in `main` gemerged. |
| `hotfix/*`    | Spoedfixes voor kritieke bugs in `main`. Worden direct na fix terug gemerged. |

Bij het maken van je commit vergeet niet een text te schrijven van wat het is + een titel
