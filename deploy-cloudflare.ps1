# ==============================================================================
# YAV.RS WEBSITE -> CLOUDFLARE PAGES TURNKEY DEPLOYER
# ==============================================================================
# Prerequisites: 
#   - yav-cli installed (yav)
#   - Node.js (for npx wrangler) or Cloudflare account
# ==============================================================================

$ErrorActionPreference = "Stop"

Write-Host ""
Write-Host "╔═══════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║     YAV.RS WEBSITE -> CLOUDFLARE PAGES DEPLOYER       ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# 1. Prerender static site via yav export
Write-Host "⏳ Étape 1/2 : Prégénération des 9 pages en HTML pur (yav export)..." -ForegroundColor Yellow
yav export --output dist --base-url https://yav.rs

if (-not (Test-Path "dist/index.html")) {
    Write-Host "❌ Erreur : dist/index.html n'a pas été généré." -ForegroundColor Red
    exit 1
}

Write-Host "✔ Étape 1 terminée avec succès ! Le dossier dist/ est prêt.`n" -ForegroundColor Green

# 2. Deploy via Wrangler CLI
Write-Host "⏳ Étape 2/2 : Téléversement vers Cloudflare Pages..." -ForegroundColor Yellow
Write-Host "ℹ Si vous n'êtes pas connecté, Cloudflare ouvrira votre navigateur une seule fois.`n" -ForegroundColor DarkGray

npx --yes wrangler pages deploy dist --project-name yav-website

Write-Host ""
Write-Host "✨ Déploiement Cloudflare Pages terminé avec succès !" -ForegroundColor Green
Write-Host "🔗 Votre site est en ligne mondialement sur le réseau Cloudflare sans aucun temps de veille !" -ForegroundColor Cyan
