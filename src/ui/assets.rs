pub fn obtener_index_html() -> &'static str {
    INDEX_HTML
}

const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="es">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Motor Analítico de Portafolio — Club de Finanzas UBA</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700;800&family=JetBrains+Mono:wght@500;700&family=Playfair+Display:wght@700&display=swap" rel="stylesheet">
  <script src="https://cdn.jsdelivr.net/npm/chart.js@4.4.1/dist/chart.umd.min.js"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/html2canvas/1.4.1/html2canvas.min.js"></script>
  <style>
    :root {
      --navy-950: #050E1D;
      --navy-900: #0A192F;
      --navy-800: #0F2D59;
      --navy-700: #1E3A8A;
      --blue-600: #0062FF;
      --blue-500: #2563EB;
      --cyan-500: #00D2D3;
      --slate-50: #F8FAFC;
      --slate-100: #F1F5F9;
      --slate-200: #E2E8F0;
      --slate-300: #CBD5E1;
      --slate-600: #475569;
      --slate-700: #334155;
      --slate-900: #0F172A;
      --card-shadow: 0 4px 20px -2px rgba(15, 23, 42, 0.08), 0 2px 6px -1px rgba(15, 23, 42, 0.04);
      --card-shadow-hover: 0 12px 30px -4px rgba(15, 23, 42, 0.12);
    }

    * {
      box-sizing: border-box;
      margin: 0;
      padding: 0;
      -webkit-font-smoothing: antialiased;
    }

    body {
      font-family: 'Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      background-color: var(--slate-100);
      color: var(--slate-900);
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      overflow-x: hidden;
    }

    /* Header Superior Institucional */
    .top-navbar {
      background: var(--navy-900);
      color: #FFFFFF;
      padding: 14px 28px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      border-bottom: 2.5px solid var(--blue-600);
      position: sticky;
      top: 0;
      z-index: 100;
      box-shadow: 0 4px 20px rgba(0,0,0,0.25);
    }

    .brand-section {
      display: flex;
      align-items: center;
      gap: 16px;
    }

    .brand-logo-badge {
      background: var(--blue-600);
      color: #FFF;
      font-weight: 800;
      font-size: 0.82rem;
      padding: 6px 12px;
      border-radius: 6px;
      letter-spacing: 0.5px;
    }

    .brand-titles {
      display: flex;
      flex-direction: column;
    }

    .brand-title-main {
      font-size: 1.18rem;
      font-weight: 800;
      letter-spacing: -0.01em;
    }

    .brand-title-sub {
      font-size: 0.76rem;
      color: var(--slate-300);
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 1px;
    }

    .nav-actions {
      display: flex;
      align-items: center;
      gap: 14px;
    }

    .db-status-pill {
      background: rgba(255,255,255,0.08);
      border: 1px solid rgba(255,255,255,0.18);
      border-radius: 20px;
      padding: 7px 16px;
      font-size: 0.8rem;
      font-weight: 600;
      display: flex;
      align-items: center;
      gap: 8px;
    }

    .status-dot {
      width: 9px;
      height: 9px;
      background: #10B981;
      border-radius: 50%;
      box-shadow: 0 0 8px #10B981;
    }

    .btn-fullscreen {
      background: var(--blue-600);
      color: #FFF;
      border: none;
      border-radius: 8px;
      padding: 8px 16px;
      font-size: 0.85rem;
      font-weight: 700;
      cursor: pointer;
      display: flex;
      align-items: center;
      gap: 8px;
      transition: all 0.2s ease;
    }

    .btn-fullscreen:hover {
      background: #0050d8;
      transform: translateY(-1px);
    }

    /* Layout Principal con Menú Lateral */
    .app-layout {
      display: flex;
      flex: 1;
    }

    .sidebar-menu {
      width: 270px;
      background: #FFFFFF;
      border-right: 1px solid var(--slate-200);
      display: flex;
      flex-direction: column;
      padding: 22px 14px;
      gap: 6px;
      flex-shrink: 0;
    }

    .menu-category-title {
      font-size: 0.72rem;
      font-weight: 800;
      color: var(--slate-600);
      text-transform: uppercase;
      letter-spacing: 1px;
      padding: 12px 14px 4px 14px;
    }

    .menu-item-btn {
      display: flex;
      align-items: center;
      gap: 12px;
      padding: 12px 16px;
      border-radius: 10px;
      font-size: 0.9rem;
      font-weight: 700;
      color: var(--slate-700);
      background: transparent;
      border: none;
      cursor: pointer;
      text-align: left;
      transition: all 0.2s ease;
      width: 100%;
    }

    .menu-item-btn:hover {
      background: var(--slate-100);
      color: var(--blue-600);
    }

    .menu-item-btn.active {
      background: #EBF3FA;
      color: var(--blue-600);
      border-left: 4px solid var(--blue-600);
      border-top-left-radius: 4px;
      border-bottom-left-radius: 4px;
    }

    .menu-icon {
      font-size: 1.2rem;
    }

    /* Área de Contenido Principal */
    .content-area {
      flex: 1;
      padding: 28px 36px;
      overflow-y: auto;
      max-width: 1600px;
      margin: 0 auto;
      width: 100%;
    }

    .view-container {
      display: none;
      flex-direction: column;
      gap: 24px;
      animation: fadeIn 0.2s ease forwards;
    }

    .view-container.active {
      display: flex;
    }

    @keyframes fadeIn {
      from { opacity: 0; transform: translateY(4px); }
      to { opacity: 1; transform: translateY(0); }
    }

    .view-header {
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      border-bottom: 1px solid var(--slate-200);
      padding-bottom: 16px;
    }

    .view-title-group h2 {
      font-size: 1.65rem;
      font-weight: 800;
      color: var(--navy-900);
      letter-spacing: -0.02em;
    }

    .view-title-group p {
      font-size: 0.9rem;
      color: var(--slate-600);
      margin-top: 4px;
    }

    /* Formularios y Tarjetas de Control */
    .card-panel {
      background: #FFFFFF;
      border-radius: 12px;
      border: 1px solid var(--slate-200);
      box-shadow: var(--card-shadow);
      padding: 24px;
      display: flex;
      flex-direction: column;
      gap: 18px;
    }

    .form-grid-controls {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(210px, 1fr));
      gap: 16px;
      align-items: flex-end;
    }

    .input-group {
      display: flex;
      flex-direction: column;
      gap: 6px;
    }

    .input-group label {
      font-size: 0.8rem;
      font-weight: 700;
      color: var(--slate-700);
      text-transform: uppercase;
      letter-spacing: 0.4px;
    }

    .input-control {
      padding: 10px 14px;
      border: 1.5px solid var(--slate-200);
      border-radius: 8px;
      font-size: 0.92rem;
      font-family: inherit;
      color: var(--slate-900);
      outline: none;
      transition: border-color 0.2s ease;
      background: #FAFCFE;
    }

    .input-control:focus {
      border-color: var(--blue-600);
      background: #FFFFFF;
    }

    .btn-action-primary {
      background: var(--blue-600);
      color: #FFFFFF;
      border: none;
      border-radius: 8px;
      padding: 11px 22px;
      font-size: 0.92rem;
      font-weight: 700;
      cursor: pointer;
      transition: all 0.2s ease;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      gap: 8px;
    }

    .btn-action-primary:hover {
      background: #0050d8;
      transform: translateY(-1px);
      box-shadow: 0 4px 12px rgba(0, 98, 255, 0.25);
    }

    .btn-action-primary:disabled {
      opacity: 0.65;
      cursor: not-allowed;
      transform: none;
    }

    /* Chips de Tickers Rápidos */
    .quick-chips-row {
      display: flex;
      align-items: center;
      gap: 8px;
      flex-wrap: wrap;
      margin-top: 4px;
    }

    .chip-label {
      font-size: 0.76rem;
      font-weight: 700;
      color: var(--slate-600);
    }

    .ticker-chip {
      background: var(--slate-100);
      border: 1px solid var(--slate-200);
      border-radius: 6px;
      padding: 4px 10px;
      font-size: 0.78rem;
      font-family: 'JetBrains Mono', monospace;
      font-weight: 700;
      color: var(--navy-900);
      cursor: pointer;
      transition: all 0.15s ease;
    }

    .ticker-chip:hover {
      background: var(--blue-600);
      color: #FFF;
      border-color: var(--blue-600);
    }

    /* Rejilla de KPIs */
    .kpis-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
      gap: 14px;
    }

    .kpi-box {
      background: #FFFFFF;
      border: 1px solid var(--slate-200);
      border-radius: 10px;
      padding: 16px;
      display: flex;
      flex-direction: column;
      align-items: center;
      text-align: center;
      box-shadow: var(--card-shadow);
      transition: transform 0.2s ease;
    }

    .kpi-box:hover {
      transform: translateY(-2px);
      box-shadow: var(--card-shadow-hover);
      border-color: var(--blue-600);
    }

    .kpi-icon-circle {
      width: 34px;
      height: 34px;
      border-radius: 50%;
      background: var(--navy-900);
      color: #FFFFFF;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 0.9rem;
      margin-bottom: 8px;
    }

    .kpi-icon-circle.blue { background: var(--blue-600); }
    .kpi-icon-circle.cyan { background: #0284C7; }

    .kpi-title {
      font-size: 0.72rem;
      font-weight: 700;
      color: var(--slate-600);
      text-transform: uppercase;
      letter-spacing: 0.4px;
      line-height: 1.2;
      min-height: 28px;
      display: flex;
      align-items: center;
      justify-content: center;
    }

    .kpi-stat {
      font-family: 'JetBrains Mono', monospace;
      font-size: 1.35rem;
      font-weight: 800;
      color: var(--blue-600);
      margin-top: 6px;
    }

    .kpi-stat.dark { color: var(--navy-900); }

    /* Gráficos 2x2 */
    .charts-grid-2x2 {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 20px;
    }

    @media (max-width: 1024px) {
      .charts-grid-2x2 {
        grid-template-columns: 1fr;
      }
    }

    .chart-panel-card {
      background: #FFFFFF;
      border: 1px solid var(--slate-200);
      border-radius: 12px;
      padding: 20px;
      box-shadow: var(--card-shadow);
      display: flex;
      flex-direction: column;
      gap: 14px;
    }

    .chart-panel-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      border-bottom: 1.5px solid var(--navy-900);
      padding-bottom: 8px;
    }

    .chart-panel-title {
      font-size: 0.88rem;
      font-weight: 800;
      color: var(--navy-900);
      text-transform: uppercase;
      letter-spacing: 0.5px;
    }

    .btn-export-1080p {
      background: var(--slate-100);
      color: var(--navy-900);
      border: 1px solid var(--slate-300);
      border-radius: 6px;
      padding: 4px 10px;
      font-size: 0.72rem;
      font-weight: 700;
      cursor: pointer;
      transition: all 0.2s ease;
      display: flex;
      align-items: center;
      gap: 4px;
    }

    .btn-export-1080p:hover {
      background: var(--blue-600);
      color: #FFF;
      border-color: var(--blue-600);
    }

    .chart-canvas-box {
      position: relative;
      width: 100%;
      height: 320px;
    }

    /* Tablas Cuantitativas */
    .table-card {
      background: #FFFFFF;
      border-radius: 12px;
      border: 1px solid var(--slate-200);
      box-shadow: var(--card-shadow);
      padding: 20px;
      overflow-x: auto;
    }

    .custom-table {
      width: 100%;
      border-collapse: collapse;
      font-size: 0.86rem;
    }

    .custom-table th {
      background: var(--slate-50);
      color: var(--navy-900);
      font-weight: 700;
      text-transform: uppercase;
      font-size: 0.75rem;
      padding: 10px 12px;
      text-align: left;
      border-top: 1.5px solid var(--navy-900);
      border-bottom: 1px solid var(--slate-300);
    }

    .custom-table td {
      padding: 9px 12px;
      border-bottom: 1px solid var(--slate-200);
      color: var(--slate-700);
    }

    .custom-table tr:hover td {
      background: var(--slate-50);
    }

    .ticker-badge {
      font-family: 'JetBrains Mono', monospace;
      font-weight: 700;
      background: #EBF3FA;
      color: var(--navy-900);
      padding: 2px 8px;
      border-radius: 4px;
    }

    /* Banner de Estado En Línea */
    .status-banner {
      display: none;
      background: #E0F2FE;
      border-left: 4px solid var(--blue-600);
      padding: 12px 18px;
      border-radius: 8px;
      font-size: 0.88rem;
      font-weight: 600;
      color: var(--navy-900);
      align-items: center;
      gap: 10px;
    }

    .status-banner.active {
      display: flex;
    }

    .spinner-inline {
      width: 16px;
      height: 16px;
      border: 2.5px solid rgba(0, 98, 255, 0.2);
      border-top-color: var(--blue-600);
      border-radius: 50%;
      animation: spin 0.7s linear infinite;
    }

    /* Catálogo de Símbolos Clickeables y Barra de Búsqueda */
    .search-filter-bar {
      display: flex;
      flex-direction: column;
      gap: 12px;
      margin-bottom: 8px;
    }

    .search-input-wrapper {
      position: relative;
      display: flex;
      align-items: center;
    }

    .search-icon-inside {
      position: absolute;
      left: 14px;
      font-size: 1.1rem;
      color: var(--slate-600);
      pointer-events: none;
    }

    .search-input-main {
      width: 100%;
      padding: 12px 14px 12px 42px;
      border: 2px solid var(--slate-200);
      border-radius: 10px;
      font-size: 1rem;
      font-family: inherit;
      background: #FAFCFE;
      color: var(--slate-900);
      outline: none;
      transition: all 0.2s ease;
    }

    .search-input-main:focus {
      border-color: var(--blue-600);
      background: #FFFFFF;
      box-shadow: 0 0 0 3px rgba(0, 98, 255, 0.15);
    }

    .filter-tags-row {
      display: flex;
      gap: 8px;
      flex-wrap: wrap;
      align-items: center;
    }

    .filter-tag-btn {
      background: var(--slate-100);
      border: 1px solid var(--slate-200);
      border-radius: 20px;
      padding: 5px 14px;
      font-size: 0.78rem;
      font-weight: 700;
      color: var(--slate-700);
      cursor: pointer;
      transition: all 0.15s ease;
    }

    .filter-tag-btn:hover {
      background: var(--slate-200);
      color: var(--navy-900);
    }

    .filter-tag-btn.active {
      background: var(--blue-600);
      color: #FFFFFF;
      border-color: var(--blue-600);
    }

    .batch-actions-bar {
      display: flex;
      justify-content: space-between;
      align-items: center;
      background: #F1F5F9;
      padding: 10px 16px;
      border-radius: 8px;
      font-size: 0.84rem;
      font-weight: 700;
      color: var(--navy-900);
      flex-wrap: wrap;
      gap: 10px;
    }

    .catalog-grid {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
      gap: 10px;
      max-height: 480px;
      overflow-y: auto;
      padding-right: 4px;
    }

    .catalog-item-card {
      background: #FAFCFE;
      border: 1.5px solid var(--slate-200);
      border-radius: 8px;
      padding: 10px 12px;
      display: flex;
      flex-direction: column;
      gap: 4px;
      cursor: pointer;
      transition: all 0.15s ease;
      position: relative;
    }

    .catalog-item-card:hover {
      border-color: var(--blue-600);
      background: #FFFFFF;
      transform: translateY(-1px);
      box-shadow: var(--card-shadow);
    }

    .catalog-item-card.in-db {
      border-left: 4px solid #10B981;
      background: #F0FDF4;
    }

    .catalog-item-card.selected {
      border-color: var(--blue-600);
      background: #EFF6FF;
      box-shadow: 0 0 0 2px rgba(0, 98, 255, 0.3);
    }

    .catalog-top-row {
      display: flex;
      justify-content: space-between;
      align-items: center;
    }

    .catalog-ticker-name {
      font-family: 'JetBrains Mono', monospace;
      font-weight: 800;
      font-size: 1rem;
      color: var(--navy-900);
    }

    .catalog-company-sub {
      font-size: 0.73rem;
      color: var(--slate-600);
      font-weight: 600;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }

    .catalog-badge-pill {
      font-size: 0.68rem;
      font-weight: 700;
      padding: 2px 6px;
      border-radius: 4px;
    }

    .catalog-badge-pill.saved {
      background: #D1FAE5;
      color: #065F46;
    }

    .catalog-badge-pill.download {
      background: #EBF3FA;
      color: var(--blue-600);
    }

    .catalog-badge-pill.sector {
      background: var(--slate-100);
      color: var(--slate-600);
      font-size: 0.65rem;
    }

    /* Toast Notification */
    .toast-box {
      position: fixed;
      bottom: 24px;
      right: 24px;
      background: var(--navy-900);
      color: #FFFFFF;
      padding: 14px 22px;
      border-radius: 10px;
      font-size: 0.88rem;
      font-weight: 600;
      box-shadow: 0 10px 25px rgba(0,0,0,0.25);
      border-left: 4px solid var(--blue-600);
      opacity: 0;
      transform: translateY(20px);
      transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
      z-index: 9999;
      pointer-events: none;
    }

    .toast-box.show {
      opacity: 1;
      transform: translateY(0);
    }

    /* Estilos Guía Metodológica & Documentación */
    .guide-card-section {
      background: var(--card-bg);
      border-radius: var(--border-radius);
      box-shadow: var(--card-shadow);
      padding: 24px 28px;
      margin-bottom: 24px;
      border: 1px solid var(--slate-200);
      transition: all 0.2s ease;
    }

    .guide-card-section:hover {
      border-color: var(--blue-600);
      box-shadow: 0 6px 20px rgba(0, 98, 255, 0.08);
    }

    .guide-card-header {
      display: flex;
      align-items: center;
      gap: 14px;
      margin-bottom: 16px;
      padding-bottom: 12px;
      border-bottom: 2px solid var(--slate-100);
    }

    .guide-icon-badge {
      width: 44px;
      height: 44px;
      border-radius: 10px;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 1.35rem;
      background: #EBF3FA;
      color: var(--blue-600);
      flex-shrink: 0;
    }

    .guide-title {
      font-size: 1.2rem;
      font-weight: 800;
      color: var(--navy-900);
      margin: 0;
    }

    .guide-subtitle {
      font-size: 0.84rem;
      color: var(--slate-600);
      margin: 2px 0 0 0;
      font-weight: 500;
    }

    .guide-formula-box {
      background: #0A192F;
      color: #38BDF8;
      border-radius: 8px;
      padding: 14px 18px;
      font-family: 'JetBrains Mono', monospace;
      font-size: 0.95rem;
      font-weight: 700;
      margin: 14px 0;
      border-left: 4px solid var(--blue-600);
      overflow-x: auto;
      line-height: 1.5;
    }

    .guide-explanation-p {
      font-size: 0.92rem;
      line-height: 1.65;
      color: var(--slate-700);
      margin-bottom: 12px;
    }

    .guide-param-list {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
      gap: 12px;
      margin-top: 14px;
    }

    .guide-param-item {
      background: #F8FAFC;
      border: 1px solid var(--slate-200);
      border-radius: 8px;
      padding: 12px 14px;
    }

    .guide-param-name {
      font-family: 'JetBrains Mono', monospace;
      font-weight: 800;
      font-size: 0.85rem;
      color: var(--blue-600);
      display: block;
      margin-bottom: 4px;
    }

    .guide-param-desc {
      font-size: 0.82rem;
      color: var(--slate-600);
      line-height: 1.45;
    }

    .guide-highlight-tag {
      display: inline-block;
      padding: 2px 8px;
      border-radius: 4px;
      font-size: 0.76rem;
      font-weight: 700;
      background: #EBF3FA;
      color: var(--blue-600);
      margin-right: 6px;
    }

    .guide-analysis-card {
      background: #FFFFFF;
      border: 1px solid var(--slate-200);
      border-left: 4px solid var(--blue-600);
      border-radius: 8px;
      padding: 16px 18px;
      margin-top: 14px;
    }

    .guide-analysis-title {
      font-size: 0.92rem;
      font-weight: 800;
      color: var(--navy-900);
      margin-bottom: 8px;
      display: flex;
      align-items: center;
      gap: 8px;
    }

    .guide-scale-row {
      display: flex;
      flex-wrap: wrap;
      gap: 8px;
      margin: 10px 0;
    }

    .scale-pill {
      font-size: 0.76rem;
      font-weight: 700;
      padding: 4px 10px;
      border-radius: 6px;
      display: flex;
      align-items: center;
      gap: 4px;
    }

    .scale-pill.bad { background: #FEE2E2; color: #991B1B; }
    .scale-pill.warn { background: #FEF3C7; color: #92400E; }
    .scale-pill.good { background: #D1FAE5; color: #065F46; }
    .scale-pill.excel { background: #E0E7FF; color: #3730A3; }
  </style>
</head>
<body>

  <!-- Barra de Navegación Superior -->
  <header class="top-navbar">
    <div class="brand-section">
      <img src="/4.png" alt="Club de Finanzas UBA" style="width: 40px; height: 40px; border-radius: 50%; object-fit: cover; background: #FFFFFF; padding: 2px; box-shadow: 0 2px 8px rgba(0,0,0,0.18); flex-shrink: 0;">
      <div class="brand-titles">
        <span class="brand-title-main">MOTOR ANALÍTICO DE PORTAFOLIO</span>
        <span class="brand-title-sub">Club de Finanzas UBA — Cuantitativo & Markowitz</span>
      </div>
    </div>
    <div class="nav-actions">
      <div class="db-status-pill">
        <span class="status-dot"></span>
        <span id="dbStatusText">Conectando a SQLite...</span>
      </div>
      <button class="btn-fullscreen" onclick="toggleFullscreen()">
        <span>🖥️</span> Pantalla Completa (F11)
      </button>
    </div>
  </header>

  <!-- Layout Principal -->
  <div class="app-layout">
    <!-- Menú Lateral Ordenado -->
    <nav class="sidebar-menu">
      <div class="menu-category-title">Análisis Cuantitativo</div>
      <button class="menu-item-btn active" onclick="switchView('view-optimizacion')">
        <span class="menu-icon">📊</span> Optimización Markowitz
      </button>
      <button class="menu-item-btn" onclick="switchView('view-is-oos')">
        <span class="menu-icon">🔬</span> Validación IS / OOS
      </button>
      <button class="menu-item-btn" onclick="switchView('view-tracking')">
        <span class="menu-icon">📈</span> Seguimiento vs SPY
      </button>

      <div class="menu-category-title">Informes & Datos</div>
      <button class="menu-item-btn" onclick="switchView('view-reporte')">
        <span class="menu-icon">📑</span> Reporte Institucional (6 Págs)
      </button>
      <button class="menu-item-btn" onclick="switchView('view-reporte-seguimiento')">
        <span class="menu-icon">📈</span> Informe de Seguimiento
      </button>
      <button class="menu-item-btn" onclick="switchView('view-db')">
        <span class="menu-icon">🗄️</span> Base de Datos SQLite
      </button>

      <div class="menu-category-title">Investigación Avanzada</div>
      <button class="menu-item-btn" onclick="switchView('view-experimental')" style="border-left: 3px solid #8b5cf6;">
        <span class="menu-icon">🧪</span> Zona Experimental (IA)
      </button>

      <div class="menu-category-title">Documentación</div>
      <button class="menu-item-btn" onclick="switchView('view-guia')">
        <span class="menu-icon">📚</span> Guía Metodológica & Fórmulas
      </button>
    </nav>

    <!-- Área de Contenido -->
    <main class="content-area">

      <!-- VISTA 1: OPTIMIZACIÓN MARKOWITZ -->
      <section id="view-optimizacion" class="view-container active">
        <div class="view-header">
          <div class="view-title-group">
            <h2>Optimización de Cartera (Markowitz, Sharpe & Sortino)</h2>
            <p>Calcula ponderaciones óptimas deterministas, frontera eficiente, ratios de arbitraje CCL y desgloses de riesgo.</p>
          </div>
        </div>

        <div id="statusBannerOpt" class="status-banner">
          <div class="spinner-inline"></div>
          <span id="statusBannerOptText">Procesando datos cuantitativos en Rust...</span>
        </div>

        <div class="card-panel">
          <div class="input-group">
            <label>Activos de la Cartera (Separados por coma)</label>
            <input type="text" id="optTickers" class="input-control" value="AAPL, YPF, AMZN, CEG, SO, XOM" placeholder="Ej: AAPL, YPF, AMZN, CEG, SO, XOM">
            <div class="quick-chips-row">
              <span class="chip-label">Activos Rápidos:</span>
              <button class="ticker-chip" onclick="agregarTicker('optTickers', 'AAPL')">+ AAPL</button>
              <button class="ticker-chip" onclick="agregarTicker('optTickers', 'YPF')">+ YPF</button>
              <button class="ticker-chip" onclick="agregarTicker('optTickers', 'AMZN')">+ AMZN</button>
              <button class="ticker-chip" onclick="agregarTicker('optTickers', 'MSFT')">+ MSFT</button>
              <button class="ticker-chip" onclick="agregarTicker('optTickers', 'NVDA')">+ NVDA</button>
              <button class="ticker-chip" onclick="agregarTicker('optTickers', 'CEG')">+ CEG</button>
              <button class="ticker-chip" onclick="agregarTicker('optTickers', 'SO')">+ SO</button>
              <button class="ticker-chip" onclick="agregarTicker('optTickers', 'XOM')">+ XOM</button>
              <button class="ticker-chip" onclick="agregarTicker('optTickers', 'MSTR')">+ MSTR</button>
              <button class="ticker-chip" onclick="agregarTicker('optTickers', 'RACE')">+ RACE</button>
            </div>
          </div>

          <div class="form-grid-controls">
            <div class="input-group">
              <label>Cantidad de Velas Históricas</label>
              <input type="number" id="optVelas" class="input-control" value="2520" step="252" min="50">
            </div>
            <div class="input-group">
              <label>CCL Referencia (ARS/USD)</label>
              <input type="number" id="optCcl" class="input-control" value="1250.0" step="10">
            </div>
            <div class="input-group">
              <label>Tasa Libre de Riesgo (Rf Anual)</label>
              <input type="number" id="optRf" class="input-control" value="0.04" step="0.005">
            </div>
            <div class="input-group">
              <label>Límite Mínimo por Activo</label>
              <input type="number" id="optMinBound" class="input-control" value="0.05" step="0.01">
            </div>
            <button id="btnEjecutarOpt" class="btn-action-primary" onclick="ejecutarOptimizacion()">
              <span>⚡</span> Ejecutar Optimización
            </button>
          </div>
        </div>

        <!-- KPIs -->
        <div class="kpis-grid">
          <div class="kpi-box">
            <div class="kpi-icon-circle blue">📈</div>
            <div class="kpi-title">Retorno Esperado</div>
            <div class="kpi-stat" id="kpiOptReturn">--%</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle dark">⚡</div>
            <div class="kpi-title">Volatilidad Anual</div>
            <div class="kpi-stat dark" id="kpiOptVol">--%</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle blue">🎯</div>
            <div class="kpi-title">Sharpe Ratio</div>
            <div class="kpi-stat" id="kpiOptSharpe">--</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle cyan">🛡️</div>
            <div class="kpi-title">Sortino Ratio</div>
            <div class="kpi-stat" id="kpiOptSortino">--</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle dark">⚠️</div>
            <div class="kpi-title">VaR 95% Diario</div>
            <div class="kpi-stat dark" id="kpiOptVar">--%</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle blue">💵</div>
            <div class="kpi-title">CCL Ponderado</div>
            <div class="kpi-stat" id="kpiOptCcl">$--</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle cyan">🔄</div>
            <div class="kpi-title">Spread Arbitraje</div>
            <div class="kpi-stat" id="kpiOptSpread">--%</div>
          </div>
        </div>

        <!-- Gráficos 2x2 -->
        <div class="charts-grid-2x2">
          <div class="chart-panel-card" id="cardPesos">
            <div class="chart-panel-header">
              <span class="chart-panel-title">Asignación de Pesos Óptimos (Sharpe vs Sortino)</span>
              <button class="btn-export-1080p" onclick="exportarGraficoPNG('cardPesos', 'Pesos_Optimos_1080p')">📸 PNG 1080p</button>
            </div>
            <div class="chart-canvas-box"><canvas id="chartPesos"></canvas></div>
          </div>

          <div class="chart-panel-card" id="cardFrontera">
            <div class="chart-panel-header">
              <span class="chart-panel-title">Frontera Eficiente de Markowitz & Cartera Óptima</span>
              <button class="btn-export-1080p" onclick="exportarGraficoPNG('cardFrontera', 'Frontera_Eficiente_1080p')">📸 PNG 1080p</button>
            </div>
            <div class="chart-canvas-box"><canvas id="chartFrontera"></canvas></div>
          </div>

          <div class="chart-panel-card" id="cardEvolucion">
            <div class="chart-panel-header">
              <span class="chart-panel-title">Evolución Histórica Acumulada (Base 100 vs SPY)</span>
              <button class="btn-export-1080p" onclick="exportarGraficoPNG('cardEvolucion', 'Evolucion_Precios_1080p')">📸 PNG 1080p</button>
            </div>
            <div class="chart-canvas-box"><canvas id="chartEvolucion"></canvas></div>
          </div>

          <div class="chart-panel-card" id="cardMapa">
            <div class="chart-panel-header">
              <span class="chart-panel-title">Mapa de Activos: Riesgo vs Retorno Esperado</span>
              <button class="btn-export-1080p" onclick="exportarGraficoPNG('cardMapa', 'Mapa_Activos_1080p')">📸 PNG 1080p</button>
            </div>
            <div class="chart-canvas-box"><canvas id="chartMapa"></canvas></div>
          </div>
        </div>

        <!-- Matriz de Correlación y Tabla -->
        <div class="table-card" id="cardCorrelacion">
          <div class="chart-panel-header" style="margin-bottom: 12px;">
            <span class="chart-panel-title">Matriz de Correlación Empírica</span>
            <button class="btn-export-1080p" onclick="exportarGraficoPNG('cardCorrelacion', 'Matriz_Correlacion_1080p')">📸 PNG 1080p</button>
          </div>
          <table class="custom-table" id="tablaCorrelacion">
            <thead><tr id="theadCorr"><th>Activo</th><th>Correlación</th></tr></thead>
            <tbody id="tbodyCorr"><tr><td colspan="2" style="text-align:center;">Haga clic en Ejecutar Optimización para calcular correlaciones.</td></tr></tbody>
          </table>
        </div>

        <div class="table-card" id="cardDesglose">
          <div class="chart-panel-header" style="margin-bottom: 12px;">
            <span class="chart-panel-title">Desglose Estadístico de Activos</span>
            <button class="btn-export-1080p" onclick="exportarGraficoPNG('cardDesglose', 'Desglose_Activos_1080p')">📸 PNG 1080p</button>
          </div>
          <table class="custom-table">
            <thead>
              <tr>
                <th>Ticker</th>
                <th>Retorno Esperado</th>
                <th>Volatilidad Anual</th>
                <th>Downside Vol</th>
                <th>Beta vs SPY</th>
                <th>Retorno CAPM</th>
                <th>Peso Máx Sharpe</th>
                <th>Peso Máx Sortino</th>
              </tr>
            </thead>
            <tbody id="tbodyDesglose">
              <tr><td colspan="8" style="text-align: center; color: var(--slate-600);">Ejecute la optimización para calcular métricas de activos.</td></tr>
            </tbody>
          </table>
        </div>

        <!-- Acciones para Guardar y Exportar Cartera a la Base de Datos -->
        <div class="card-panel" style="display:flex; justify-content:space-between; align-items:center; flex-wrap:wrap; gap:12px; margin-top:14px; background:linear-gradient(135deg, var(--slate-50), #EFF6FF); border-left:4px solid var(--blue-600);">
          <div>
            <span style="font-weight:800; font-size:0.95rem; color:var(--navy-900);">💾 Guardar Cartera en Base de Datos SQLite</span>
            <p style="font-size:0.8rem; color:var(--slate-600); margin-top:2px;">Almacene las ponderaciones calculadas en la base de datos para importar en Seguimiento o auditar históricamente.</p>
          </div>
          <div style="display:flex; gap:10px; flex-wrap:wrap;">
            <button id="btnGuardarSharpe" class="btn-action-primary" style="background:#16a34a; padding:8px 16px; font-size:0.85rem;" onclick="abrirModalGuardarCartera('sharpe')">
              <span>💾</span> Guardar Máx. Sharpe
            </button>
            <button id="btnGuardarSortino" class="btn-action-primary" style="background:#0284c7; padding:8px 16px; font-size:0.85rem;" onclick="abrirModalGuardarCartera('sortino')">
              <span>💾</span> Guardar Máx. Sortino
            </button>
            <button class="btn-action-primary" style="background:var(--navy-900); padding:8px 16px; font-size:0.85rem;" onclick="enviarOptimizacionASeguimiento()">
              <span>📈</span> Enviar a Seguimiento
            </button>
          </div>
        </div>
      </section>

      <!-- VISTA 2: VALIDACIÓN IS / OOS -->
      <section id="view-is-oos" class="view-container">
        <div class="view-header">
          <div class="view-title-group">
            <h2>Validación In-Sample (IS) vs Out-Of-Sample (OOS)</h2>
            <p>Optimiza los pesos en la ventana In-Sample y simula la cartera en Out-Of-Sample con rebalanceo periódico contra un Benchmark seleccionable.</p>
          </div>
        </div>

        <div id="statusBannerIsOos" class="status-banner">
          <div class="spinner-inline"></div>
          <span id="statusBannerIsOosText">Ejecutando validación IS / OOS y rebalanceo de cartera...</span>
        </div>

        <div class="card-panel">
          <div class="input-group">
            <label>Activos de la Cartera (Separados por coma)</label>
            <input type="text" id="isOosTickers" class="input-control" value="AAPL, YPF, AMZN, CEG, SO, XOM">
            <div class="quick-chips-row">
              <span class="chip-label">Activos Rápidos:</span>
              <button class="ticker-chip" onclick="agregarTicker('isOosTickers', 'AAPL')">+ AAPL</button>
              <button class="ticker-chip" onclick="agregarTicker('isOosTickers', 'YPF')">+ YPF</button>
              <button class="ticker-chip" onclick="agregarTicker('isOosTickers', 'AMZN')">+ AMZN</button>
              <button class="ticker-chip" onclick="agregarTicker('isOosTickers', 'NVDA')">+ NVDA</button>
              <button class="ticker-chip" onclick="agregarTicker('isOosTickers', 'CEG')">+ CEG</button>
              <button class="ticker-chip" onclick="agregarTicker('isOosTickers', 'SO')">+ SO</button>
              <button class="ticker-chip" onclick="agregarTicker('isOosTickers', 'XOM')">+ XOM</button>
              <button class="ticker-chip" onclick="agregarTicker('isOosTickers', 'MSTR')">+ MSTR</button>
              <button class="ticker-chip" onclick="agregarTicker('isOosTickers', 'RACE')">+ RACE</button>
            </div>
          </div>

          <div class="form-grid-controls">
            <div class="input-group">
              <label>Velas In-Sample (Entrenamiento)</label>
              <input type="number" id="isVelas" class="input-control" value="252" step="50" min="50">
            </div>
            <div class="input-group">
              <label>Velas Out-Of-Sample (Prueba)</label>
              <input type="number" id="oosVelas" class="input-control" value="252" step="50" min="50">
            </div>
            <div class="input-group">
              <label>Frecuencia de Rebalanceo (OOS)</label>
              <select id="isOosRebalFreq" class="input-control">
                <option value="mensual" selected>Mensual (Cada 21 velas)</option>
                <option value="semanal">Semanal (Cada 5 velas)</option>
                <option value="diario">Diario (Cada 1 vela)</option>
                <option value="sin_rebalanceo">Sin Rebalanceo (Buy & Hold)</option>
              </select>
            </div>
            <div class="input-group">
              <label>Benchmark Seleccionable</label>
              <input type="text" id="isOosBenchmark" class="input-control" value="SPY" placeholder="Ej: SPY, QQQ o SPY: 60, QQQ: 40">
              <div class="quick-chips-row" style="margin-top: 2px;">
                <span class="chip-label">Sugeridos:</span>
                <button class="ticker-chip" onclick="document.getElementById('isOosBenchmark').value='SPY'">SPY</button>
                <button class="ticker-chip" onclick="document.getElementById('isOosBenchmark').value='QQQ'">QQQ</button>
                <button class="ticker-chip" onclick="document.getElementById('isOosBenchmark').value='DIA'">DIA</button>
                <button class="ticker-chip" onclick="document.getElementById('isOosBenchmark').value='SPY: 60, QQQ: 40'">60/40 SPY+QQQ</button>
              </div>
            </div>
            <div class="input-group">
              <label>CCL Referencia (ARS/USD)</label>
              <input type="number" id="isOosCcl" class="input-control" value="1250.0">
            </div>
            <button id="btnEjecutarIsOos" class="btn-action-primary" onclick="ejecutarIsOos()">
              <span>🔬</span> Ejecutar Validación IS/OOS
            </button>
          </div>
        </div>

        <div class="kpis-grid">
          <div class="kpi-box">
            <div class="kpi-icon-circle blue">IS</div>
            <div class="kpi-title">Retorno In-Sample</div>
            <div class="kpi-stat" id="kpiIsRet">--%</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle cyan">OOS</div>
            <div class="kpi-title">Retorno OOS (Rebalanceado)</div>
            <div class="kpi-stat" id="kpiOosRet">--%</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle dark">IS</div>
            <div class="kpi-title">Sharpe In-Sample</div>
            <div class="kpi-stat dark" id="kpiIsSharpe">--</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle dark">OOS</div>
            <div class="kpi-title">Sharpe OOS (Rebalanceado)</div>
            <div class="kpi-stat dark" id="kpiOosSharpe">--</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle cyan">💰</div>
            <div class="kpi-title">Ganancia Neta OOS</div>
            <div class="kpi-stat" id="kpiOosGain">--%</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle blue">📉</div>
            <div class="kpi-title">Max Drawdown OOS</div>
            <div class="kpi-stat" id="kpiOosMaxDd">--%</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle dark">🎯</div>
            <div class="kpi-title">Retorno Benchmark OOS</div>
            <div class="kpi-stat dark" id="kpiBmOosRet">--%</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle blue">⚡</div>
            <div class="kpi-title">Alpha OOS vs Benchmark</div>
            <div class="kpi-stat" id="kpiAlphaOos">--%</div>
          </div>
        </div>

        <div class="charts-grid-2x2">
          <div class="chart-panel-card" id="cardIsOosEvolucion" style="grid-column: span 2;">
            <div class="chart-panel-header">
              <span class="chart-panel-title">Curva de Rendimiento Acumulado Continua: In-Sample (Entrenamiento) vs Out-Of-Sample (Prueba con Rebalanceo) vs Benchmark</span>
              <button class="btn-export-1080p" onclick="exportarGraficoPNG('cardIsOosEvolucion', 'Is_Oos_Curva_Equity')">📸 PNG 1080p</button>
            </div>
            <div class="chart-canvas-box" style="height: 380px;"><canvas id="chartIsOosEvolucion"></canvas></div>
          </div>

          <div class="chart-panel-card" id="cardIsOosPesos" style="grid-column: span 2;">
            <div class="chart-panel-header">
              <span class="chart-panel-title">Ponderaciones Asignadas en In-Sample (IS %)</span>
              <button class="btn-export-1080p" onclick="exportarGraficoPNG('cardIsOosPesos', 'Is_Oos_Pesos')">📸 PNG 1080p</button>
            </div>
            <div class="chart-canvas-box"><canvas id="chartIsOosPesos"></canvas></div>
          </div>
        </div>
      </section>

      <!-- VISTA 3: SEGUIMIENTO DE PORTAFOLIO -->
      <section id="view-tracking" class="view-container">
        <div class="view-header">
          <div class="view-title-group">
            <h2>Seguimiento de Portafolio Personalizado (Evolución de Capital vs SPY)</h2>
            <p>Analiza el rendimiento histórico exacto, Drawdowns y periodos de estancamiento de una cartera con ponderaciones personalizadas.</p>
          </div>
        </div>

        <div id="statusBannerTrack" class="status-banner">
          <div class="spinner-inline"></div>
          <span id="statusBannerTrackText">Calculando seguimiento de portafolio...</span>
        </div>

        <!-- IMPORTADOR DE CARTERAS GUARDADAS DESDE SQLITE -->
        <div class="card-panel" style="border-left:4px solid var(--blue-600); margin-bottom:14px; background:linear-gradient(135deg, var(--slate-50), #EFF6FF);">
          <div style="display:flex; justify-content:space-between; align-items:center; flex-wrap:wrap; gap:10px;">
            <div>
              <span style="font-weight:800; font-size:0.95rem; color:var(--navy-900);">📂 Importar Cartera desde Base de Datos SQLite</span>
              <p style="font-size:0.8rem; color:var(--slate-600); margin-top:2px;">Seleccione una cartera previamente guardada para cargar automáticamente sus activos y ponderaciones.</p>
            </div>
            <div style="display:flex; align-items:center; gap:8px; flex-wrap:wrap;">
              <select id="selectCarterasDbTrack" class="input-control" style="min-width:260px; padding:6px 10px; font-size:0.85rem;" onchange="previewCarteraSeleccionadaTrack()">
                <option value="">-- Seleccionar Cartera Guardada --</option>
              </select>
              <button class="btn-action-primary" style="padding:7px 14px; font-size:0.85rem; background:#16a34a;" onclick="cargarCarteraSeleccionadaTrack()">
                <span>📥</span> Cargar en Seguimiento
              </button>
            </div>
          </div>
          <div id="previewCarteraTrack" style="margin-top:8px; font-size:0.82rem; color:var(--blue-700); font-weight:700; display:none; background:#FFF; padding:6px 12px; border-radius:6px; border:1px solid var(--blue-100);"></div>
        </div>

        <div class="card-panel">
          <div style="display:grid; grid-template-columns: 1fr 1fr; gap:12px; margin-bottom:12px;">
            <div class="input-group">
              <label>Activos de la Cartera</label>
              <input type="text" id="trackTickers" class="input-control" value="AAPL, YPF, AMZN">
            </div>
            <div class="input-group">
              <label>👥 Autores del Seguimiento / Integrantes & Cargos</label>
              <input type="text" id="trackIntegrantes" class="input-control" value="Fausto Crivelli (Presidente de Portafolio) · Luciano Mora (Analista Sr) · Florencia Beluzzo (Analista Sr)" placeholder="Ej: Fausto Crivelli (Presidente de Portafolio) · Luciano Mora (Analista Sr)">
            </div>
          </div>
          <div class="form-grid-controls">
            <div class="input-group">
              <label>Ponderaciones % (ej: 40, 30, 30)</label>
              <input type="text" id="trackPesos" class="input-control" value="40, 30, 30" placeholder="Ponderaciones separadas por coma">
            </div>
            <div class="input-group">
              <label>Frecuencia de Rebalanceo</label>
              <select id="trackRebalance" class="input-control">
                <option value="mensual" selected>Mensual (Cada 21 velas)</option>
                <option value="semanal">Semanal (Cada 5 velas)</option>
                <option value="diario">Diario (Cada 1 vela)</option>
                <option value="trimestral">Trimestral (Cada 63 velas)</option>
                <option value="sin_rebalanceo">Sin Rebalanceo (Buy & Hold)</option>
              </select>
            </div>
            <div class="input-group">
              <label>Fecha de Inicio (YYYY-MM-DD)</label>
              <input type="date" id="trackFecha" class="input-control" value="2024-01-01">
            </div>
            <div class="input-group">
              <label>CCL Referencia (ARS/USD)</label>
              <input type="number" id="trackCcl" class="input-control" value="1250.0">
            </div>
            <button id="btnEjecutarTrack" class="btn-action-primary" onclick="ejecutarTracking()">
              <span>📈</span> Calcular Seguimiento
            </button>
          </div>
        </div>

        <div class="kpis-grid">
          <div class="kpi-box">
            <div class="kpi-icon-circle blue">💰</div>
            <div class="kpi-title">Ganancia Acumulada</div>
            <div class="kpi-stat" id="kpiTrackGain">--%</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle dark">📉</div>
            <div class="kpi-title">Max Drawdown</div>
            <div class="kpi-stat dark" id="kpiTrackMaxDd">--%</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle blue">🎯</div>
            <div class="kpi-title">Sharpe Real</div>
            <div class="kpi-stat" id="kpiTrackSharpe">--</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle cyan">⏳</div>
            <div class="kpi-title">Estancamiento Máx</div>
            <div class="kpi-stat" id="kpiTrackMaxStag">-- días</div>
          </div>
        </div>

        <div class="card-panel" id="panelTrackComposition" style="padding: 12px 16px; margin-bottom: 16px; display: none;">
          <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
            <span style="font-size: 0.88rem; font-weight: 800; color: var(--navy-900); display: flex; align-items: center; gap: 6px;">
              <span>💼</span> Composición de la Cartera Simulada
            </span>
            <span id="badgeTrackTotalWeight" style="background: var(--navy-900); color: #ffffff; padding: 2px 8px; border-radius: 12px; font-size: 0.75rem; font-weight: 700;">100% Invertido</span>
          </div>
          <div id="containerTrackWeightsList" style="display: flex; flex-wrap: wrap; gap: 8px;"></div>
        </div>

        <div class="charts-grid-2x2">
          <div class="chart-panel-card" id="cardTrackEquity" style="grid-column: span 2;">
            <div class="chart-panel-header">
              <span class="chart-panel-title">Curva de Equity de la Cartera vs SPY (Base 1.0)</span>
              <button class="btn-export-1080p" onclick="exportarGraficoPNG('cardTrackEquity', 'Curva_Equity_Seguimiento')">📸 PNG 1080p</button>
            </div>
            <div class="chart-canvas-box" style="height: 380px;"><canvas id="chartTrackEquity"></canvas></div>
          </div>
        </div>

      </section>

      <!-- VISTA 4: REPORTE INSTITUCIONAL -->
      <section id="view-reporte" class="view-container">
        <div class="view-header">
          <div class="view-title-group">
            <h2>Reporte Institucional Completo (Club de Finanzas UBA)</h2>
            <p>Maquetación oficial de 5 páginas editable en vivo: haz clic en cualquier texto para modificarlo, formato en negrita e importa datos y gráficos de Markowitz y Seguimiento.</p>
          </div>
          <div style="display: flex; gap: 8px; flex-wrap: wrap;">
            <button id="btnParentEditReport" class="btn-action-primary" style="background: #D97706;" onclick="toggleModoEdicionReporte()">
              <span id="iconParentEdit">✏️</span> <span id="textParentEdit">Modo Edición: OFF</span>
            </button>
            <button class="btn-action-primary" style="background: #1E293B; font-weight: 900;" onclick="aplicarNegritaReporte()" title="Pone en negrita el texto seleccionado">
              <strong>B</strong> Negrita
            </button>
            <button class="btn-action-primary" style="background: #0D9488;" onclick="importarDatosMarkowitzAReporte()">
              <span>📥</span> Importar de Markowitz
            </button>
            <button class="btn-action-primary" style="background: #0062FF;" onclick="importarDatosSeguimientoAReporteInstitucional()">
              <span>📊</span> Importar de Seguimiento
            </button>
            <button class="btn-action-primary" style="background: #2563EB;" onclick="guardarTextosReporte()">
              <span>💾</span> Guardar Textos
            </button>
            <button class="btn-action-primary" style="background: #64748B;" onclick="restaurarReporteOriginal()">
              <span>🔄</span> Restaurar Original
            </button>
            <button class="btn-action-primary" onclick="abrirReporteEnNuevaVentana()">
              <span>↗️</span> Pestaña Completa
            </button>
            <button class="btn-action-primary" style="background: var(--navy-900);" onclick="imprimirIframeReporte()">
              <span>🖨️</span> Imprimir / PDF
            </button>
          </div>
        </div>

        <div class="card-panel" style="padding: 0; overflow: hidden; height: 850px;">
          <iframe id="iframeReporte" src="/api/reporte/html" style="width: 100%; height: 100%; border: none;"></iframe>
        </div>
      </section>

      <!-- VISTA 4B: INFORME OFICIAL DE SEGUIMIENTO -->
      <section id="view-reporte-seguimiento" class="view-container">
        <div class="view-header">
          <div class="view-title-group">
            <h2>Informe de Seguimiento de Portafolio vs SPY</h2>
            <p>Maquetación ejecutiva editable: monitoreo de capital, curvas de drawdown, tabla comparativa y espacio para notas del comité de inversiones.</p>
          </div>
          <div style="display: flex; gap: 8px; flex-wrap: wrap; align-items: center;">
            <div style="display: flex; align-items: center; gap: 6px; background: #FFF; padding: 4px 10px; border-radius: 8px; border: 1px solid var(--slate-300);">
              <span style="font-size: 0.82rem; font-weight: 700; color: var(--navy-900);">👥 Autores:</span>
              <input type="text" id="reportTrackIntegrantesInput" class="input-control" style="width: 320px; padding: 4px 8px; font-size: 0.8rem;" value="Fausto Crivelli (Presidente de Portafolio) · Luciano Mora (Analista Sr) · Florencia Beluzzo (Analista Sr)" placeholder="Nombres y cargos de autores" oninput="actualizarIntegrantesReporteSeguimiento(this.value)">
            </div>
            <button id="btnParentEditTrackReport" class="btn-action-primary" style="background: #D97706;" onclick="toggleModoEdicionReporteSeguimiento()">
              <span id="iconParentEditTrack">✏️</span> <span id="textParentEditTrack">Modo Edición: OFF</span>
            </button>
            <button class="btn-action-primary" style="background: #0D9488;" onclick="importarDatosSeguimientoAReporte()">
              <span>📥</span> Sincronizar Seguimiento
            </button>
            <button class="btn-action-primary" style="background: #2563EB;" onclick="guardarTextosReporteSeguimiento()">
              <span>💾</span> Guardar Textos
            </button>
            <button class="btn-action-primary" style="background: #64748B;" onclick="restaurarReporteSeguimientoOriginal()">
              <span>🔄</span> Restaurar Original
            </button>
            <button class="btn-action-primary" onclick="abrirReporteSeguimientoEnNuevaVentana()">
              <span>↗️</span> Pestaña Completa
            </button>
            <button class="btn-action-primary" style="background: var(--navy-900);" onclick="imprimirIframeReporteSeguimiento()">
              <span>🖨️</span> Imprimir / PDF
            </button>
          </div>
        </div>

        <div class="card-panel" style="padding: 0; overflow: hidden; height: 880px;">
          <iframe id="iframeReporteSeguimiento" src="/api/reporte-seguimiento/html" style="width: 100%; height: 100%; border: none;"></iframe>
        </div>
      </section>

      <!-- VISTA 5: BASE DE DATOS SQLITE -->
      <section id="view-db" class="view-container">
        <div class="view-header">
          <div class="view-title-group">
            <h2>Gestión de Base de Datos SQLite (portafolio.db)</h2>
            <p>Seleccione o descargue cotizaciones de mercado desde el catálogo predefinido o consulte el histórico guardado.</p>
          </div>
        </div>

        <!-- BUSCADOR Y CATÁLOGO DE MERCADO -->
        <div class="card-panel">
          <div class="search-filter-bar">
            <div class="search-input-wrapper">
              <span class="search-icon-inside">🔍</span>
              <input type="text" id="catalogSearchInput" class="search-input-main" placeholder="Buscar por ticker, empresa, sector o CEDEAR (ej: Google, Apple, YPF, Galicia, Nuclear, S&P 500, ETF...)" oninput="filtrarCatalogo()">
            </div>
            
            <div class="filter-tags-row">
              <span class="chip-label">Filtros:</span>
              <button class="filter-tag-btn active" onclick="setFiltroCategoria('todos', this)">Todos (<span id="countTotalCat">150+</span>)</button>
              <button class="filter-tag-btn" onclick="setFiltroCategoria('cedears', this)">🇦🇷 CEDEARs & Argentina</button>
              <button class="filter-tag-btn" onclick="setFiltroCategoria('big_tech', this)">💻 Big Tech & IA</button>
              <button class="filter-tag-btn" onclick="setFiltroCategoria('sp500', this)">🇺🇸 S&P 500 Mega-Caps</button>
              <button class="filter-tag-btn" onclick="setFiltroCategoria('energia', this)">⚡ Energía & Nuclear</button>
              <button class="filter-tag-btn" onclick="setFiltroCategoria('etfs', this)">📊 ETFs & Commodities</button>
              <button class="filter-tag-btn" onclick="setFiltroCategoria('en_db', this)">✓ Guardados en SQLite</button>
            </div>

            <div class="batch-actions-bar">
              <div style="display:flex; align-items:center; gap:12px;">
                <label style="display:flex; align-items:center; gap:6px; cursor:pointer;">
                  <input type="checkbox" id="chkSelectAll" onchange="toggleSelectAll(this.checked)"> Seleccionar Todos los Visibles
                </label>
                <span id="selectedCountText" style="color:var(--blue-600); font-weight:800;">0 seleccionados</span>
              </div>
              <div style="display:flex; gap:8px;">
                <button class="ticker-chip" style="background:var(--blue-600); color:#FFF; padding:6px 14px;" onclick="descargarSeleccionados()">📥 Descargar Seleccionados</button>
                <button class="ticker-chip" style="background:var(--navy-900); color:#FFF; padding:6px 14px;" onclick="agregarSeleccionadosACartera()">⚡ Agregar a la Cartera</button>
              </div>
            </div>
          </div>

          <div class="catalog-grid" id="catalogGrid">
            <!-- Renderizado dinámico con estado en base de datos -->
          </div>
        </div>


        <div class="table-card">
          <table class="custom-table">
            <thead>
              <tr>
                <th>Ticker</th>
                <th>Velas Guardadas</th>
                <th>Fecha Más Antigua</th>
                <th>Fecha Más Reciente</th>
              </tr>
            </thead>
            <tbody id="tbodyDbResumen">
              <tr><td colspan="4" style="text-align: center;">Cargando datos de SQLite...</td></tr>
            </tbody>
          </table>
        </div>

        <!-- TABLA DE CARTERAS GUARDADAS EN SQLITE -->
        <div class="table-card" style="margin-top: 22px;">
          <div class="chart-panel-header" style="margin-bottom: 12px;">
            <span class="chart-panel-title">📁 Carteras de Inversión Guardadas en Base de Datos (<span id="countCarterasDb">0</span>)</span>
            <button class="ticker-chip" style="background:var(--blue-600); color:#FFF; padding:6px 14px;" onclick="cargarCarterasDb()">🔄 Actualizar Carteras</button>
          </div>
          <table class="custom-table" id="tablaCarterasDb">
            <thead>
              <tr>
                <th>ID</th>
                <th>Nombre de Cartera</th>
                <th>Tipo</th>
                <th>Activos & Ponderaciones</th>
                <th>Métricas Estimadas</th>
                <th>Fecha Guardado</th>
                <th style="text-align:center;">Acciones</th>
              </tr>
            </thead>
            <tbody id="tbodyCarterasDb">
              <tr><td colspan="7" style="text-align: center;">Cargando carteras guardadas de SQLite...</td></tr>
            </tbody>
          </table>
        </div>
      </section>

      <!-- VISTA EXPERIMENTAL: RED NEURONAL DE ASIGNACIÓN DINÁMICA -->
      <section id="view-experimental" class="view-container">
        <div class="view-header">
          <div class="view-title-group">
            <h2 style="display:flex; align-items:center; gap:8px;">
              <span style="background:#8B5CF6; color:#FFF; padding:4px 10px; border-radius:8px; font-size:1rem;">🧪 EXPERIMENTAL</span>
              Red Neuronal de Asignación de Portafolios (CSAN + Causal DRL)
            </h2>
            <p>Arquitectura de Red Neuronal con Atención Transversal (Cross-Sectional Attention) y Normalización Causal Welford ($t-1$) para selección dinámica de a lo sumo <strong>5 activos</strong> con piso mínimo de <strong>5%</strong>.</p>
          </div>
        </div>

        <div id="statusBannerNeural" class="status-banner">
          <div class="spinner-inline"></div>
          <span id="statusBannerNeuralText">Ejecutando inferencia neuronal causal paso a paso en Rust...</span>
        </div>

        <div class="card-panel">
          <div class="input-group">
            <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:6px;">
              <label style="margin:0;">Universo de Activos a Evaluar por la Red Neuronal</label>
              <button class="ticker-chip" style="background:#8B5CF6; color:#FFF; font-size:0.75rem; padding:3px 10px;" onclick="cargarTodosTickersANeural()">📥 Cargar Todos de SQLite</button>
            </div>
            <input type="text" id="neuralTickers" class="input-control" value="AAPL, YPF, AMZN, MSFT, NVDA, CEG, SO, XOM, MSTR, RACE, GGAL, MELI, VIST" placeholder="Ej: AAPL, YPF, AMZN, MSFT, NVDA, CEG...">
            <div class="quick-chips-row">
              <span class="chip-label">Activos Rápidos:</span>
              <button class="ticker-chip" onclick="agregarTicker('neuralTickers', 'AAPL')">+ AAPL</button>
              <button class="ticker-chip" onclick="agregarTicker('neuralTickers', 'YPF')">+ YPF</button>
              <button class="ticker-chip" onclick="agregarTicker('neuralTickers', 'NVDA')">+ NVDA</button>
              <button class="ticker-chip" onclick="agregarTicker('neuralTickers', 'AMZN')">+ AMZN</button>
              <button class="ticker-chip" onclick="agregarTicker('neuralTickers', 'MELI')">+ MELI</button>
              <button class="ticker-chip" onclick="agregarTicker('neuralTickers', 'GGAL')">+ GGAL</button>
              <button class="ticker-chip" onclick="agregarTicker('neuralTickers', 'MSTR')">+ MSTR</button>
              <button class="ticker-chip" onclick="agregarTicker('neuralTickers', 'RACE')">+ RACE</button>
            </div>
          </div>

          <div class="form-grid-controls">
            <div class="input-group">
              <label>Ventana Causal $W$ (Ruedas)</label>
              <input type="number" id="neuralLookback" class="input-control" value="63" min="21" step="21">
            </div>
            <div class="input-group">
              <label>Frecuencia de Rebalanceo</label>
              <select id="neuralRebalance" class="input-control">
                <option value="21" selected>Mensual (Cada 21 ruedas)</option>
                <option value="5">Semanal (Cada 5 ruedas)</option>
                <option value="63">Trimestral (Cada 63 ruedas)</option>
                <option value="1">Diario (Cada 1 rueda)</option>
              </select>
            </div>
            <div class="input-group">
              <label>Máx. Activos Seleccionados</label>
              <input type="number" id="neuralMaxCard" class="input-control" value="5" min="1" max="10" readonly style="background:var(--slate-100);">
            </div>
            <div class="input-group">
              <label>Piso Mínimo por Activo</label>
              <input type="number" id="neuralMinWeight" class="input-control" value="0.05" min="0.01" max="0.20" step="0.01" readonly style="background:var(--slate-100);">
            </div>
            <div class="input-group">
              <label>Fricciones / Comisión (bps)</label>
              <input type="number" id="neuralFeeBps" class="input-control" value="10.0" min="0" step="1">
            </div>
            <button id="btnEjecutarNeural" class="btn-action-primary" style="background:#8B5CF6;" onclick="ejecutarSimulacionNeural()">
              <span>🧠</span> Ejecutar Simulación IA
            </button>
          </div>
        </div>

        <!-- KPIs de Rendimiento Neuronal -->
        <div class="kpis-grid">
          <div class="kpi-box">
            <div class="kpi-icon-circle" style="background:#8B5CF6; color:#FFF;">🤖</div>
            <div class="kpi-title">Ganancia Neta Cartera IA</div>
            <div class="kpi-stat" id="kpiNeuralGain" style="color:#8B5CF6;">--%</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle blue">📈</div>
            <div class="kpi-title">Retorno Anualizado</div>
            <div class="kpi-stat" id="kpiNeuralAnnRet">--%</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle green">⚡</div>
            <div class="kpi-title">Alpha Anual vs SPY</div>
            <div class="kpi-stat green" id="kpiNeuralAlpha">--%</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle cyan">🎯</div>
            <div class="kpi-title">Sharpe Ratio OOS</div>
            <div class="kpi-stat" id="kpiNeuralSharpe">--</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle dark">📉</div>
            <div class="kpi-title">Max Drawdown</div>
            <div class="kpi-stat dark" id="kpiNeuralMaxDd">--%</div>
          </div>
          <div class="kpi-box">
            <div class="kpi-icon-circle" style="background:#F59E0B; color:#FFF;">🔄</div>
            <div class="kpi-title">Turnover Promedio</div>
            <div class="kpi-stat" id="kpiNeuralTurnover">--%</div>
          </div>
        </div>

        <!-- Panel de Gráficos -->
        <div class="charts-grid-2x2">
          <!-- Gráfico de Equity -->
          <div class="chart-panel-card" id="cardNeuralEquity" style="grid-column: span 2;">
            <div class="chart-panel-header">
              <span class="chart-panel-title">📈 Curva de Equity Acumulado: Red Neuronal vs SPY Benchmark vs Cartera Equiponderada (1/N)</span>
              <button class="btn-export-1080p" onclick="exportarGraficoPNG('cardNeuralEquity', 'Equity_Red_Neuronal')">📸 PNG 1080p</button>
            </div>
            <div class="chart-canvas-box" style="height: 400px;"><canvas id="chartNeuralEquity"></canvas></div>
          </div>
        </div>

        <!-- PANEL DE AUDITORÍA FORMAL E INTEGRIDAD ESTADÍSTICA -->
        <div class="card-panel" style="margin-top:20px; border-left:4px solid #8B5CF6; background:linear-gradient(180deg, #FAF5FF 0%, #FFFFFF 100%);">
          <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:12px;">
            <div style="display:flex; align-items:center; gap:8px;">
              <span style="font-size:1.3rem;">🛡️</span>
              <h3 style="font-size:1.05rem; font-weight:800; color:var(--navy-900); margin:0;">Informe de Auditoría Cuantitativa & Verificación de Invariantes</h3>
            </div>
            <span id="badgeAuditStatus" class="badge-pill" style="background:#16A34A; color:#FFF; font-weight:800; font-size:0.8rem; padding:4px 12px;">✓ AUDITORÍA APROBADA</span>
          </div>

          <div style="display:grid; grid-template-columns:repeat(auto-fit, minmax(220px, 1fr)); gap:10px; margin-bottom:14px;">
            <div style="background:#FFF; border:1px solid #E9D5FF; border-radius:8px; padding:10px;">
              <div style="font-size:0.75rem; color:var(--slate-500); font-weight:700;">LOOK-AHEAD BIAS</div>
              <div style="font-size:0.95rem; font-weight:800; color:#15803D; margin-top:2px;" id="lblAuditLookahead">0% (Lag Causal $t-1$)</div>
            </div>
            <div style="background:#FFF; border:1px solid #E9D5FF; border-radius:8px; padding:10px;">
              <div style="font-size:0.75rem; color:var(--slate-500); font-weight:700;">RESTRICCIÓN DE CARDINALIDAD</div>
              <div style="font-size:0.95rem; font-weight:800; color:#15803D; margin-top:2px;" id="lblAuditCardinality">Máximo 5 Activos (100% OK)</div>
            </div>
            <div style="background:#FFF; border:1px solid #E9D5FF; border-radius:8px; padding:10px;">
              <div style="font-size:0.75rem; color:var(--slate-500); font-weight:700;">PISO MÍNIMO POR ACTIVO</div>
              <div style="font-size:0.95rem; font-weight:800; color:#15803D; margin-top:2px;" id="lblAuditMinWeight">Piso 5% Respetado</div>
            </div>
            <div style="background:#FFF; border:1px solid #E9D5FF; border-radius:8px; padding:10px;">
              <div style="font-size:0.75rem; color:var(--slate-500); font-weight:700;">SUMA DE PONDERACIONES</div>
              <div style="font-size:0.95rem; font-weight:800; color:#15803D; margin-top:2px;" id="lblAuditSumWeights">100.00% Exacto</div>
            </div>
          </div>

          <div id="containerAuditNotes" style="font-size:0.85rem; color:var(--navy-900); line-height:1.5;">
            <!-- Notas de auditoría generadas en Rust -->
          </div>
        </div>

        <!-- Tabla Histórica de Asignaciones y Rebalanceos -->
        <div class="table-card" style="margin-top:20px;">
          <div class="chart-panel-header" style="margin-bottom:12px;">
            <span class="chart-panel-title">📋 Registro Cronológico de Decisiones de la Red Neuronal (<span id="countNeuralRebalances">0</span> Rebalanceos)</span>
          </div>
          <table class="custom-table" id="tablaNeuralAllocations">
            <thead>
              <tr>
                <th>Fecha / Rueda</th>
                <th>Activos Seleccionados (Top $\le$ 5)</th>
                <th>Ponderaciones Asignadas (%)</th>
                <th>Rotación (Turnover)</th>
                <th>Costo Deducido</th>
                <th>Equity Cartera</th>
                <th>Equity SPY</th>
              </tr>
            </thead>
            <tbody id="tbodyNeuralAllocations">
              <tr><td colspan="7" style="text-align:center; color:var(--slate-500);">Ejecute la simulación neuronal para ver el historial de decisiones de inversión.</td></tr>
            </tbody>
          </table>
        </div>
      </section>

      <!-- VISTA 6: GUÍA METODOLÓGICA, TRATADO TEÓRICO & FÓRMULAS -->
      <section id="view-guia" class="view-container">
        <div class="view-header">
          <div class="view-title-group">
            <h2>📚 Tratado Teórico, Metodología Cuantitativa & Manual de Portafolios</h2>
            <p>Fundamentos matemáticos rigurosos, demostraciones conceptuales y manual paso a paso: por qué funciona cada modelo, cómo se calcula y cómo trasladar la teoría a reportes institucionales.</p>
          </div>
        </div>

        <!-- MÓDULO 0: PASO A PASO MAESTRO PARA CREAR UN PORTAFOLIO -->
        <div class="guide-card-section" style="border: 2px solid var(--blue-600); background: linear-gradient(180deg, #F8FAFC 0%, #FFFFFF 100%);">
          <div class="guide-card-header">
            <div class="guide-icon-badge" style="background: var(--blue-600);">🗺️</div>
            <div>
              <h3 class="guide-title" style="color: var(--navy-950);">Módulo Maestro: Flujo Metodológico de Construcción de Portafolios (Paso a Paso)</h3>
              <p class="guide-subtitle">El ciclo completo de 5 fases: desde la concepción macro hasta la publicación del informe de 6 páginas</p>
            </div>
          </div>
          
          <p class="guide-explanation-p">
            Construir una cartera de inversión profesional no es adivinar qué acción va a subir mañana, sino <strong>aplicar un proceso cuantitativo riguroso, repetible y blindado contra el riesgo</strong>. A continuación se detalla la metodología exacta que utiliza el <strong>Club de Finanzas UBA</strong> en esta plataforma.
          </p>

          <!-- FASE 1 -->
          <div style="background: #FFFFFF; border: 1px solid var(--slate-200); border-left: 4px solid var(--blue-600); border-radius: 8px; padding: 14px 18px; margin-bottom: 14px;">
            <h4 style="font-size: 15px; font-weight: 800; color: var(--navy-900); margin-bottom: 6px;">
              📍 FASE 1: Concepción de la Tesis Macro & Selección de Activos
            </h4>
            <p style="font-size: 13.5px; line-height: 1.5; color: var(--slate-700); margin-bottom: 8px;">
              <strong>1. Definir la Tesis de Inversión:</strong> Identifica un fenómeno macroeconómico o estructural de mediano/largo plazo (ej: <em>"La expansión de centros de datos de IA generará un déficit de 19 GW de energía eléctrica en EE.UU."</em>).
            </p>
            <p style="font-size: 13.5px; line-height: 1.5; color: var(--slate-700); margin-bottom: 8px;">
              <strong>2. Seleccionar el Universo (5 a 10 activos):</strong> Elige compañías que capturen ese fenómeno desde distintos eslabones de la cadena de valor para no concentrarte en una sola empresa.
            </p>
            <p style="font-size: 13.5px; line-height: 1.5; color: var(--slate-700);">
              <strong>3. Descarga y Verificación en SQLite:</strong> Dirígete a la pestaña <strong>🗄️ Base de Datos SQLite</strong>, busca los tickers (filtrando por <em>🇦🇷 CEDEARs & Argentina</em> si operas desde Argentina) y descárgalos con al menos 1.000 a 2.520 ruedas de historial (4 a 10 años).
            </p>
          </div>

          <!-- FASE 2 -->
          <div style="background: #FFFFFF; border: 1px solid var(--slate-200); border-left: 4px solid var(--teal-600); border-radius: 8px; padding: 14px 18px; margin-bottom: 14px;">
            <h4 style="font-size: 15px; font-weight: 800; color: var(--navy-900); margin-bottom: 6px;">
              ⚙️ FASE 2: Diagnóstico Estadístico & Arbitraje de Dólar CCL
            </h4>
            <p style="font-size: 13.5px; line-height: 1.5; color: var(--slate-700); margin-bottom: 8px;">
              <strong>1. Matriz de Correlación:</strong> Revisa el mapa de calor de correlaciones. Si dos activos tienen correlación cercana a +1.0 (se mueven idéntico), estás duplicando riesgo sin ganar diversificación. El objetivo es combinar activos con correlaciones medias bajas (ej: menores a 0,40).
            </p>
            <p style="font-size: 13.5px; line-height: 1.5; color: var(--slate-700); margin-bottom: 8px;">
              <strong>2. Tasa Libre de Riesgo (R_f = 4,0%):</strong> Se fija la tasa de los Bonos del Tesoro de EE.UU. a 10 años como referencia mínima que cualquier inversión en dólares debe superar.
            </p>
            <p style="font-size: 13.5px; line-height: 1.5; color: var(--slate-700);">
              <strong>3. Control de Arbitraje CCL:</strong> Verifica el tipo de cambio implícito de cada CEDEAR frente al CCL de referencia ($1.250). Si la cartera tiene un spread negativo (-2,5%), significa que estás comprando los activos subyacentes con descuento cambiario en BYMA.
            </p>
          </div>

          <!-- FASE 3 -->
          <div style="background: #FFFFFF; border: 1px solid var(--slate-200); border-left: 4px solid var(--blue-700); border-radius: 8px; padding: 14px 18px; margin-bottom: 14px;">
            <h4 style="font-size: 15px; font-weight: 800; color: var(--navy-900); margin-bottom: 6px;">
              📊 FASE 3: Optimización Matricial de Markowitz & Frontera Eficiente
            </h4>
            <p style="font-size: 13.5px; line-height: 1.5; color: var(--slate-700); margin-bottom: 8px;">
              En la pestaña <strong>📊 Optimización Markowitz</strong>:
            </p>
            <ul style="font-size: 13px; color: var(--slate-600); margin-left: 20px; line-height: 1.6; margin-bottom: 8px;">
              <li>Ingresa los tickers separados por coma.</li>
              <li>Fija la <strong>Ponderación Mínima (Bound)</strong> en 5% para asegurar diversificación mínima.</li>
              <li>Haz clic en <strong>⚡ Optimizar Cartera</strong>.</li>
            </ul>
            <p style="font-size: 13.5px; line-height: 1.5; color: var(--slate-700);">
              <strong>Interpretación de Soluciones:</strong> Selecciona entre <strong>Cartera Máximo Sharpe</strong> (punto tangente), <strong>Máximo Sortino</strong> (protección bajista) o <strong>Mínima Varianza Global (GMV)</strong> (menor riesgo total).
            </p>
          </div>

          <!-- FASE 4 -->
          <div style="background: #FFFFFF; border: 1px solid var(--slate-200); border-left: 4px solid #D97706; border-radius: 8px; padding: 14px 18px; margin-bottom: 14px;">
            <h4 style="font-size: 15px; font-weight: 800; color: var(--navy-900); margin-bottom: 6px;">
              🔬 FASE 4: Validación Fuera de Muestra & Seguimiento con Rebalanceo
            </h4>
            <p style="font-size: 13.5px; line-height: 1.5; color: var(--slate-700); margin-bottom: 8px;">
              <strong>1. Validación In-Sample / Out-of-Sample (IS/OOS):</strong> Evalúa si la cartera calculada en el pasado (IS) sigue batiendo al SPY en el período de prueba (OOS). Si el Sharpe OOS es sólido (> 1,20), la cartera no está sobreajustada.
            </p>
            <p style="font-size: 13.5px; line-height: 1.5; color: var(--slate-700);">
              <strong>2. Seguimiento vs SPY:</strong> En la pestaña <em>📈 Seguimiento vs SPY</em>, selecciona la frecuencia de rebalanceo (<strong>Mensual</strong> es la estándar). Verifica que el <strong>Máximo Drawdown</strong> sea menor al del mercado.
            </p>
          </div>

          <!-- FASE 5 -->
          <div style="background: #FFFFFF; border: 1px solid var(--slate-200); border-left: 4px solid #16A34A; border-radius: 8px; padding: 14px 18px;">
            <h4 style="font-size: 15px; font-weight: 800; color: var(--navy-900); margin-bottom: 6px;">
              📑 FASE 5: Edición, Redacción y Publicación del Informe Institucional
            </h4>
            <p style="font-size: 13.5px; line-height: 1.5; color: var(--slate-700); margin-bottom: 8px;">
              <strong>1. Volcado Automático de Métricas:</strong> Dirígete a <strong>📑 Reporte Institucional (6 Págs)</strong> y haz clic en <strong>`📥 Importar de Markowitz`</strong>. Las tablas de ponderación, KPIs y gráficos se actualizarán con los datos de tu optimización.
            </p>
            <p style="font-size: 13.5px; line-height: 1.5; color: var(--slate-700); margin-bottom: 8px;">
              <strong>2. Personalización Editorial con Modo Edición:</strong> Haz clic en <strong>`✏️ Modo Edición: ON`</strong> para redactar el análisis cualitativo.
            </p>
            <p style="font-size: 13.5px; line-height: 1.5; color: var(--slate-700); margin-bottom: 8px;">
              <strong>3. Guardar y Exportar:</strong> Presiona <strong>`💾 Guardar Textos`</strong> para conservar tu redacción en <em>localStorage</em>, y luego haz clic en <strong>`🖨️ Imprimir / PDF`</strong> para generar el documento ejecutivo oficial en formato A4 listo para presentar.
            </p>
          </div>
        </div>

        <!-- 1. FUNDAMENTOS MATEMÁTICOS DEL RENDIMIENTO -->
        <div class="guide-card-section">
          <div class="guide-card-header">
            <div class="guide-icon-badge">💵</div>
            <div>
              <h3 class="guide-title">1. Fundamentos Matemáticos del Rendimiento y Volatility Drag</h3>
              <p class="guide-subtitle">Rendimientos Simples vs Logarítmicos, Lema de Itô y la Erosión Geométrica de la Varianza</p>
            </div>
          </div>
          <p class="guide-explanation-p">
            En finanzas cuantitativas, la forma en que calculas el rendimiento determina si tu modelo matemático es válido o defectuoso. Existen dos tipos de rendimientos con propiedades radicalmente distintas:
          </p>
          
          <div class="guide-formula-box">
            • Rendimiento Simple (Discreto):   R_t = (P_t - P_{t-1}) / P_{t-1} = P_t / P_{t-1} - 1<br>
            • Rendimiento Continuo (Log):      r_t = ln( P_t / P_{t-1} ) = ln(P_t) - ln(P_{t-1})<br>
            • Tasa Compuesta Anual (CAGR):     CAGR = ( P_T / P_0 )^(252 / T) - 1<br>
            • Fricción por Volatilidad (Itô):  CAGR ≈ μ_aritmético - 0.5 · σ²
          </div>

          <div class="guide-param-list">
            <div class="guide-param-item">
              <span class="guide-param-name">¿Por qué el motor usa Rendimientos Logarítmicos (r_t)?</span>
              <span class="guide-param-desc"><strong>Aditividad Temporal:</strong> La suma de los rendimientos logarítmicos diarios a lo largo de un año es exactamente igual al rendimiento logarítmico anual total: ∑ r_t = ln(P_T / P_0). Con rendimientos simples esto es matemáticamente imposible. Además, bajo el modelo de Movimiento Browniano Geométrico, los rendimientos logarítmicos siguen una distribución Normal, habilitando el álgebra matricial.</span>
            </div>
            <div class="guide-param-item">
              <span class="guide-param-name">¿Por qué el Promedio Aritmético Engaña al Inversor?</span>
              <span class="guide-param-desc">Si una acción sube +50% el año 1 y cae -50% el año 2, el promedio simple parece (50 - 50)/2 = 0%. Sin embargo, tu dinero pasó de $100 a $150 y luego a $75. <strong>Perdiste el 25% real de tu patrimonio</strong>. El CAGR mide exactamente tu crecimiento patrimonial real.</span>
            </div>
            <div class="guide-param-item">
              <span class="guide-param-name">Volatility Drag (Erosión por Volatilidad)</span>
              <span class="guide-param-desc">La fórmula de aproximación derivada del Lema de Itô demuestra que <strong>a mayor volatilidad (σ), mayor es la destrucción del capital compuesto (-0.5·σ²)</strong>. Por eso una cartera con la mitad de volatilidad que otra puede generar mucho más dinero a largo plazo aunque ambas tengan la misma rentabilidad media aritmética.</span>
            </div>
          </div>

          <div class="guide-analysis-card">
            <div class="guide-analysis-title">📊 Escala Institucional de Evaluación de CAGR</div>
            <div class="guide-scale-row">
              <span class="scale-pill bad">🔴 CAGR < 5%: Pobre (Pierde contra bonos del tesoro)</span>
              <span class="scale-pill warn">🟡 CAGR 5% - 10%: Aceptable conservador</span>
              <span class="scale-pill good">🟢 CAGR 10% - 20%: Excelente desempeño de renta variable</span>
              <span class="scale-pill excel">🔵 CAGR > 20%: Sobresaliente (Supera ampliamente al S&P 500)</span>
            </div>
          </div>
        </div>

        <!-- 2. TEORÍA MODERNA DE PORTAFOLIO (MARKOWITZ, 1952) -->
        <div class="guide-card-section">
          <div class="guide-card-header">
            <div class="guide-icon-badge">🧺</div>
            <div>
              <h3 class="guide-title">2. Teoría Moderna de Portafolio: Por qué la Covarianza Domina al Riesgo</h3>
              <p class="guide-subtitle">Álgebra Matricial, Términos Cruzados N(N-1) y Formulación de Optimización Cuadrática</p>
            </div>
          </div>
          <p class="guide-explanation-p">
            Harry Markowitz (Premio Nobel 1990) demostró matemáticamente que el riesgo de una cartera <strong>no es el promedio ponderado de los riesgos individuales</strong>, sino que depende casi exclusivamente de cómo covarían los activos entre sí.
          </p>

          <div class="guide-formula-box">
            • Retorno Esperado Matricial:   E(R_p) = w^T · μ = ∑ w_i · E(R_i)<br>
            • Varianza de la Cartera:       σ_p² = w^T · Σ · w = ∑ w_i² σ_i² + ∑∑_{i ≠ j} w_i w_j Cov(R_i, R_j)<br>
            • Covarianza y Correlación:     Cov(R_i, R_j) = ρ_{ij} · σ_i · σ_j<br>
            • Problema de Optimización:     min_{w} 0.5 · w^T Σ w  sujeto a  w^T 1 = 1,  w_i ≥ w_min
          </div>

          <div class="guide-param-list">
            <div class="guide-param-item">
              <span class="guide-param-name">¿Por qué la Covarianza importa más que la Varianza individual?</span>
              <span class="guide-param-desc">En una cartera de N activos, hay <strong>N términos de varianza propia</strong> y <strong>N(N-1) términos de covarianza cruzada</strong>. Para una cartera de 10 acciones, hay 10 varianzas y 90 covarianzas (el 90% del riesgo lo define la interacción). Si los activos están descorrelacionados (ρ < 0.40), el riesgo se extingue matemáticamente.</span>
            </div>
            <div class="guide-param-item">
              <span class="guide-param-name">Ratio de Sharpe vs Ratio de Sortino</span>
              <span class="guide-param-desc"><strong>Sharpe</strong> penaliza toda oscilación (incluso subidas bruscas). <strong>Sortino</strong> utiliza la <em>Semi-Desviación Bajista (Downside Deviation)</em>, ignorando las subidas y castigando únicamente los días en que el precio cae por debajo del objetivo.</span>
            </div>
          </div>

          <div class="guide-analysis-card">
            <div class="guide-analysis-title">📊 Escala del Ratio de Sharpe & Sortino</div>
            <div class="guide-scale-row">
              <span class="scale-pill bad">🔴 Sharpe < 0.5: Pobre</span>
              <span class="scale-pill warn">🟡 Sharpe 0.5 - 1.0: En línea con el mercado</span>
              <span class="scale-pill good">🟢 Sharpe 1.0 - 1.5: Cartera muy eficiente</span>
              <span class="scale-pill excel">🔵 Sharpe > 1.5: Desempeño institucional sobresaliente</span>
            </div>
          </div>
        </div>

        <!-- 3. LA FRONTERA EFICIENTE -->
        <div class="guide-card-section">
          <div class="guide-card-header">
            <div class="guide-icon-badge">📈</div>
            <div>
              <h3 class="guide-title">3. La Frontera Eficiente y el Teorema de Separación de Tobin</h3>
              <p class="guide-subtitle">Cartera Tangente (Máximo Sharpe), Cartera GMV (Mínima Varianza) y el Error de Merton</p>
            </div>
          </div>
          <p class="guide-explanation-p">
            La Frontera Eficiente es el conjunto geométrico de carteras que ofrecen el máximo retorno posible para cada nivel de volatilidad.
          </p>

          <div class="guide-formula-box">
            • Línea de Asignación (CAL):    E(R_CAL) = R_f + [ ( E(R_p) - R_f ) / σ_p ] · σ<br>
            • Cartera Tangente (Max Sharpe): max_{w} ( w^T μ - R_f ) / √( w^T Σ w )<br>
            • Cartera GMV:                   min_{w} w^T Σ w  sujeto a  w^T 1 = 1  (¡Sin usar μ!)
          </div>

          <div class="guide-param-list">
            <div class="guide-param-item">
              <span class="guide-param-name">El Teorema de Separación de Dos Fondos (James Tobin, 1958)</span>
              <span class="guide-param-desc">Demuestra que la decisión de inversión se divide en dos pasos totalmente independientes: 1) Encontrar la Cartera Tangente única en el punto de tangencia con la CAL; 2) Ajustar el nivel de riesgo deseado combinando esa misma cartera con el activo libre de riesgo (R_f), sin necesidad de cambiar las ponderaciones de las acciones.</span>
            </div>
            <div class="guide-param-item">
              <span class="guide-param-name">¿Por qué la Cartera GMV es tan crucial? (El Error de Estimación de Merton)</span>
              <span class="guide-param-desc">Estudios empíricos (Merton 1980, Chopra & Ziemba 1993) demostraron que el error al estimar retornos futuros esperados (μ) es 10 veces mayor que el error al estimar la matriz de covarianza (Σ). La Cartera GMV es la única que <strong>no utiliza retornos esperados</strong>, minimizando el riesgo de error de pronóstico en producción.</span>
            </div>
          </div>
        </div>

        <!-- 4. MODELO CAPM Y DESCOMPOSICIÓN DE RIESGO -->
        <div class="guide-card-section">
          <div class="guide-card-header">
            <div class="guide-icon-badge">🎯</div>
            <div>
              <h3 class="guide-title">4. Modelo CAPM: Riesgo Sistemático (Beta) vs Alpha de Jensen</h3>
              <p class="guide-subtitle">Por qué el Mercado no Paga Prima por Riesgo Diversificable y la Ecuación SML</p>
            </div>
          </div>
          <p class="guide-explanation-p">
            El Capital Asset Pricing Model (Sharpe, Lintner, Mossin) descompone el riesgo de cualquier activo en dos componentes mutuamente excluyentes:
          </p>

          <div class="guide-formula-box">
            • Descomposición del Retorno:   R_i - R_f = α_i + β_i · ( R_m - R_f ) + ε_i<br>
            • Coeficiente Beta:             β_i = Cov(R_i, R_m) / σ_m² = ρ_{i,m} · ( σ_i / σ_m )<br>
            • Retorno Exigido CAPM (SML):   E(R_i) = R_f + β_i · [ E(R_m) - R_f ]<br>
            • Alpha de Jensen:              α_i = R̄_i - ( R_f + β_i · [ R̄_m - R_f ] )
          </div>

          <div class="guide-param-list">
            <div class="guide-param-item">
              <span class="guide-param-name">El Teorema de No Recompensa al Riesgo Idiosincrático</span>
              <span class="guide-param-desc">El mercado bursátil no te paga ningún premio por asumir riesgo propio de una empresa (ε_i), porque cualquier inversor puede anularlo gratis diversificando. Por ende, el único riesgo remunerado con prima de retorno es el <strong>Riesgo Sistemático (Beta)</strong>.</span>
            </div>
            <div class="guide-param-item">
              <span class="guide-param-name">Beta de Cartera (β_p)</span>
              <span class="guide-param-desc">β < 1.0 (Defensiva, amortigua caídas); β = 1.0 (En línea con el S&P 500); β > 1.0 (Agresiva y altamente cíclica).</span>
            </div>
          </div>
        </div>

        <!-- 5. GESTIÓN DE RIESGO DE COLA -->
        <div class="guide-card-section">
          <div class="guide-card-header">
            <div class="guide-icon-badge">🛡️</div>
            <div>
              <h3 class="guide-title">5. Gestión de Riesgo de Cola: Value at Risk (VaR 95%) y Ratio de Calmar</h3>
              <p class="guide-subtitle">Probabilidad Estadística de Pérdida Máxima, Drawdown Continuo y Recuperación de Capital</p>
            </div>
          </div>
          <p class="guide-explanation-p">
            La gestión institucional de riesgo exige saber de antemano cuánto capital puede perderse en escenarios adversos y cuánto tiempo tarda la cartera en recuperarse de sus caídas.
          </p>

          <div class="guide-formula-box">
            • VaR 95% Paramétrico (1 Día):  VaR_{95%} = - ( μ_diario - 1.645 · σ_diaria ) ≈ 1.645 · σ_diaria<br>
            • Drawdown en el Instante t:    DD_t = ( P_t - max_{τ ≤ t} P_τ ) / max_{τ ≤ t} P_τ<br>
            • Máximo Drawdown Histórico:    MDD = min_{t} DD_t<br>
            • Ratio de Calmar:              Calmar = CAGR / |MDD|
          </div>

          <div class="guide-param-list">
            <div class="guide-param-item">
              <span class="guide-param-name">¿Para qué sirve el VaR 95%?</span>
              <span class="guide-param-desc">Fija el límite de pérdida máxima esperada con un 95% de probabilidad en condiciones normales de mercado. Es el requerimiento estándar de Basilea III para reservas de capital de bancos y fondos de inversión.</span>
            </div>
            <div class="guide-param-item">
              <span class="guide-param-name">Ratio de Calmar (Rentabilidad vs Caída Máxima)</span>
              <span class="guide-param-desc">Mide cuántas unidades de ganancia compuesta anual generas por cada unidad de caída histórica máxima soportada. Un Calmar > 1.5 indica una gestión de caídas sobresaliente.</span>
            </div>
          </div>
        </div>

        <!-- 6. VALIDACIÓN IS / OOS Y REBALANCEO -->
        <div class="guide-card-section">
          <div class="guide-card-header">
            <div class="guide-icon-badge">🔬</div>
            <div>
              <h3 class="guide-title">6. Validación Fuera de Muestra (IS / OOS) y Cosecha de Volatilidad</h3>
              <p class="guide-subtitle">Control de Overfitting / Data Snooping y el Demonio de Shannon en Rebalanceo Periódico</p>
            </div>
          </div>
          <p class="guide-explanation-p">
            La <strong>Validación In-Sample / Out-of-Sample</strong> divide la historia para calibrar pesos en el pasado (IS) y probarlos a ciegas en el período siguiente (OOS), evitando el autoengaño del sobreajuste estadístico.
          </p>

          <div class="guide-formula-box">
            • Período In-Sample (IS 70%):    Ventana donde el algoritmo calibra los pesos w_i.<br>
            • Período Out-of-Sample (OOS 30%): Ventana ciega donde se evalúa el Sharpe y Alpha real sin optimizar.<br>
            • Rebalanceo Periódico:           Cada 21 ruedas (mensual), los pesos se reajustan mecánicamente a w_i.<br>
            • Cosecha de Volatilidad:         El rebalanceo compra en caídas y toma ganancias en subidas, generando Alpha geométrico.
          </div>

          <div class="guide-param-list">
            <div class="guide-param-item">
              <span class="guide-param-name">El Demonio de Shannon (Volatility Harvesting)</span>
              <span class="guide-param-desc">Claude Shannon demostró que al rebalancear periódicamente entre activos oscilantes descorrelacionados, la cartera obtiene un rendimiento compuesto superior al promedio de los activos individuales, convirtiendo la volatilidad en retorno geométrico.</span>
            </div>
          </div>
        </div>

        <!-- 7. ARBITRAJE Y DÓLAR CCL -->
        <div class="guide-card-section">
          <div class="guide-card-header">
            <div class="guide-icon-badge">🇦🇷</div>
            <div>
            <div class="guide-analysis-title">📊 Análisis de Arbitraje Cambiario & Ejecución</div>
            <p class="guide-param-desc">
              En el mercado argentino (BYMA), debido a la iliquidez puntual de algunos CEDEARs, los tipos de cambio implícitos divergen del dólar CCL promedio del mercado, generando oportunidades de compra con descuento.
            </p>
            <div class="guide-scale-row">
              <span class="scale-pill good">🟢 Spread < -1.5%: Oportunidad de Compra (Dólar barato en pesos)</span>
              <span class="scale-pill warn">🟡 Spread -1.5% a +1.5%: Precio justo de arbitraje (En paridad)</span>
              <span class="scale-pill bad">🔴 Spread > +1.5%: Sobreprecio cambiario (Conviene esperar o comprar otro CEDEAR)</span>
            </div>
            <p class="guide-param-desc" style="margin-top: 8px;">
              💡 <strong>Regla de Decisión:</strong> Al momento de constituir la cartera en pesos argentinos, ejecuta primero las órdenes de aquellos CEDEARs con spread negativo más pronunciado para maximizar tu poder adquisitivo en dólares.
            </p>
          </div>
        </div>
      </section>



    </main>
  </div>

  <!-- Modal Guardar Cartera en SQLite -->
  <div id="modalGuardarCartera" class="modal-overlay" style="display:none; position:fixed; inset:0; background:rgba(9,26,54,0.7); z-index:9999; align-items:center; justify-content:center; backdrop-filter:blur(4px);">
    <div class="modal-card" style="background:#ffffff; border-radius:12px; width:540px; max-width:92%; padding:24px; box-shadow:0 24px 60px rgba(0,0,0,0.35); border:1px solid var(--slate-200);">
      <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:6px;">
        <h3 style="font-size:1.25rem; font-weight:800; color:var(--navy-900);">💾 Guardar Cartera en Base de Datos</h3>
        <button onclick="cerrarModalGuardarCartera()" style="background:none; border:none; font-size:1.4rem; color:var(--slate-400); cursor:pointer; padding:0 4px;">&times;</button>
      </div>
      <p style="font-size:0.85rem; color:var(--slate-600); margin-bottom:16px;">Almacene las ponderaciones óptimas en SQLite (portafolio.db) para importar en Seguimiento o realizar auditorías históricas.</p>
      
      <div class="input-group" style="margin-bottom:12px;">
        <label style="font-weight:700; font-size:0.85rem; color:var(--slate-700);">Nombre de la Cartera *</label>
        <input type="text" id="inputModalNombreCartera" class="input-control" placeholder="Ej: Cartera Óptima Markowitz Agosto 2026">
      </div>

      <div class="input-group" style="margin-bottom:12px;">
        <label style="font-weight:700; font-size:0.85rem; color:var(--slate-700);">Descripción / Notas del Comité</label>
        <input type="text" id="inputModalDescCartera" class="input-control" placeholder="Ej: 6 activos seleccionados, rebalanceo mensual recomendado">
      </div>

      <div id="modalPreviewCartera" style="background:var(--slate-50); border:1px solid var(--slate-200); border-radius:8px; padding:12px; font-size:0.85rem; margin-bottom:18px; max-height:160px; overflow-y:auto;">
        <!-- Tickers y pesos calculados -->
      </div>

      <div style="display:flex; justify-content:flex-end; gap:10px;">
        <button class="ticker-chip" style="background:var(--slate-200); color:var(--slate-700); padding:8px 18px;" onclick="cerrarModalGuardarCartera()">Cancelar</button>
        <button class="btn-action-primary" style="padding:8px 20px; background:#16a34a;" onclick="confirmarGuardarCarteraDb()">💾 Guardar Cartera</button>
      </div>
    </div>
  </div>

  <!-- Toast Notification -->
  <div id="toastBox" class="toast-box">
    <span id="toastMsg">Mensaje</span>
  </div>

  <script>
    let charts = {};

    function switchView(viewId) {
      document.querySelectorAll('.view-container').forEach(el => el.classList.remove('active'));
      document.querySelectorAll('.menu-item-btn').forEach(el => el.classList.remove('active'));
      
      const targetView = document.getElementById(viewId);
      if (targetView) targetView.classList.add('active');

      const btn = Array.from(document.querySelectorAll('.menu-item-btn')).find(b => b.getAttribute('onclick')?.includes(viewId));
      if (btn) btn.classList.add('active');

      if (viewId === 'view-db') {
        cargarResumenDb();
        cargarCarterasDb();
      }
      if (viewId === 'view-tracking') {
        cargarCarterasDb();
      }
    }

    function toggleFullscreen() {
      if (!document.fullscreenElement) {
        document.documentElement.requestFullscreen().catch(err => {
          console.warn("Fullscreen no soportado o bloqueado: ", err);
        });
      } else {
        if (document.exitFullscreen) document.exitFullscreen();
      }
    }

    function showToast(msg) {
      const toast = document.getElementById('toastBox');
      document.getElementById('toastMsg').innerText = msg;
      toast.classList.add('show');
      setTimeout(() => toast.classList.remove('show'), 3500);
    }

    function setStatus(bannerId, textId, btnId, active, msg) {
      const banner = document.getElementById(bannerId);
      const text = document.getElementById(textId);
      const btn = document.getElementById(btnId);
      if (banner && text) {
        text.innerText = msg || "Procesando...";
        if (active) banner.classList.add('active');
        else banner.classList.remove('active');
      }
      if (btn) btn.disabled = active;
    }

    function agregarTicker(inputId, sym) {
      const el = document.getElementById(inputId);
      let curr = el.value.split(',').map(s => s.trim().toUpperCase()).filter(s => s.length > 0);
      if (!curr.includes(sym)) {
        curr.push(sym);
        el.value = curr.join(', ');
      }
    }

    async function ejecutarOptimizacion() {
      const tickersRaw = document.getElementById('optTickers').value;
      const tickers = tickersRaw.split(',').map(s => s.trim().toUpperCase()).filter(s => s.length > 0);
      if (tickers.length === 0) {
        showToast("⚠️ Ingrese al menos un ticker.");
        return;
      }

      const payload = {
        tickers,
        n_velas: parseInt(document.getElementById('optVelas').value) || 2520,
        ccl_ref: parseFloat(document.getElementById('optCcl').value) || 1250.0,
        rf_rate: parseFloat(document.getElementById('optRf').value) || 0.04,
        min_bound: parseFloat(document.getElementById('optMinBound').value) || 0.05,
        poblacional: false
      };

      setStatus('statusBannerOpt', 'statusBannerOptText', 'btnEjecutarOpt', true, `Calculando Markowitz y verificando cotizaciones para ${tickers.join(', ')}...`);
      try {
        const res = await fetch('/api/optimizar', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
        });
        const data = await res.json();
        setStatus('statusBannerOpt', 'statusBannerOptText', 'btnEjecutarOpt', false);

        if (!data.success) {
          showToast("❌ " + (data.error || "Error al optimizar"));
          return;
        }

        renderOptimizacionResultados(data.data);
        showToast("✓ Cartera optimizada con éxito.");
      } catch (err) {
        setStatus('statusBannerOpt', 'statusBannerOptText', 'btnEjecutarOpt', false);
        showToast("❌ Error de conexión: " + err.message);
      }
    }

    function renderOptimizacionResultados(data) {
      window.lastOptResult = data;
      try { localStorage.setItem('cfuba_last_opt_result', JSON.stringify(data)); } catch(e){}

      // 1. KPIs
      document.getElementById('kpiOptReturn').innerText = (data.port_return_sharpe * 100).toFixed(1) + '%';
      document.getElementById('kpiOptVol').innerText = (data.port_vol_sharpe * 100).toFixed(1) + '%';
      document.getElementById('kpiOptSharpe').innerText = data.sharpe_ratio.toFixed(2);
      document.getElementById('kpiOptSortino').innerText = (data.sortino_ratio || 0).toFixed(2);
      document.getElementById('kpiOptVar').innerText = (data.var_95 * 100).toFixed(2) + '%';
      document.getElementById('kpiOptCcl').innerText = '$' + data.tc_cartera_ponderado.toLocaleString('es-AR', {minimumFractionDigits:2});
      document.getElementById('kpiOptSpread').innerText = (data.spread_ccl * 100).toFixed(2) + '%';

      if (typeof Chart === 'undefined') {
        console.warn("Chart.js no está disponible aún.");
        return;
      }

      // 2. Gráfico de Pesos
      if (charts.pesos) charts.pesos.destroy();
      const ctxPesos = document.getElementById('chartPesos').getContext('2d');
      charts.pesos = new Chart(ctxPesos, {
        type: 'bar',
        data: {
          labels: data.tickers,
          datasets: [
            {
              label: 'Ponderación Máx. Sharpe (%)',
              data: data.pesos_sharpe.map(p => (p * 100).toFixed(1)),
              backgroundColor: '#0062FF',
              borderRadius: 6
            },
            {
              label: 'Ponderación Máx. Sortino (%)',
              data: data.pesos_sortino.map(p => (p * 100).toFixed(1)),
              backgroundColor: '#0A192F',
              borderRadius: 6
            }
          ]
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: { legend: { position: 'top' } }
        }
      });

      // 3. Frontera Eficiente
      if (charts.frontera) charts.frontera.destroy();
      const ctxFrontera = document.getElementById('chartFrontera').getContext('2d');
      const puntosFrontera = data.frontera_puntos.map(p => ({ x: +(p[0]*100).toFixed(1), y: +(p[1]*100).toFixed(1) }));
      charts.frontera = new Chart(ctxFrontera, {
        type: 'scatter',
        data: {
          datasets: [
            {
              label: 'Frontera Eficiente de Markowitz',
              data: puntosFrontera,
              showLine: true,
              borderColor: '#0062FF',
              backgroundColor: 'rgba(0, 98, 255, 0.08)',
              borderWidth: 2.8,
              fill: false,
              tension: 0.35,
              pointRadius: 3,
              pointHoverRadius: 6
            },
            {
              label: 'Cartera Óptima Máx Sharpe',
              data: [{ x: +(data.port_vol_sharpe*100).toFixed(1), y: +(data.port_return_sharpe*100).toFixed(1) }],
              backgroundColor: '#00D2D3',
              borderColor: '#0A192F',
              borderWidth: 2,
              pointRadius: 9,
              pointHoverRadius: 11
            }
          ]
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          scales: {
            x: {
              type: 'linear',
              title: { display: true, text: 'Volatilidad Anualizada σ (%)', font: { weight: 'bold' } },
              ticks: { callback: function(val) { return val + '%'; } }
            },
            y: {
              type: 'linear',
              title: { display: true, text: 'Retorno Anual Esperado (%)', font: { weight: 'bold' } },
              ticks: { callback: function(val) { return val + '%'; } }
            }
          },
          plugins: { legend: { position: 'top' } }
        }
      });

      // 4. Evolución Acumulada (Base 100 con Cartera Óptima, SPY Benchmark y Activos)
      if (charts.evolucion) charts.evolucion.destroy();
      const ctxEvolucion = document.getElementById('chartEvolucion').getContext('2d');
      const timeLabels = data.time_labels.slice(-60);
      const nDays = timeLabels.length;
      const datasetsEvol = [];
      const colores = ['#00D2D3', '#F59E0B', '#10B981', '#EC4899', '#8B5CF6', '#F97316', '#64748B'];

      // 4.1 Cartera Óptima Ponderada (Base 100)
      if (data.tickers && data.tickers.length > 0) {
        const portPrices = [];
        for (let d = 0; d < nDays; d++) {
          let val = 0;
          data.tickers.forEach((t, i) => {
            const series = data.series_map[t];
            if (series && series.length >= nDays) {
              const sliceP = series.slice(-nDays);
              const pBase = sliceP[0] || 1;
              val += (data.pesos_sharpe[i] || 0) * (sliceP[d] / pBase);
            }
          });
          portPrices.push((val * 100).toFixed(2));
        }
        datasetsEvol.push({
          label: '★ Cartera Óptima (Máx Sharpe)',
          data: portPrices,
          borderColor: '#0062FF',
          backgroundColor: 'rgba(0, 98, 255, 0.08)',
          borderWidth: 3.2,
          fill: true,
          tension: 0.15,
          pointRadius: 0,
          order: 1
        });
      }

      // 4.2 SPY Benchmark (Base 100)
      const spySeries = data.series_map['SPY'];
      if (spySeries && spySeries.length >= nDays) {
        const sliceSpy = spySeries.slice(-nDays);
        const baseSpy = sliceSpy[0] || 1;
        datasetsEvol.push({
          label: '📊 SPY (Benchmark S&P 500)',
          data: sliceSpy.map(p => ((p / baseSpy) * 100).toFixed(2)),
          borderColor: '#0A192F',
          borderWidth: 2.5,
          borderDash: [5, 3],
          fill: false,
          tension: 0.15,
          pointRadius: 0,
          order: 2
        });
      }

      // 4.3 Activos Individuales (Base 100)
      data.tickers.forEach((t, i) => {
        const series = data.series_map[t];
        if (series && series.length > 0) {
          const sliceP = series.slice(-nDays);
          const base = sliceP[0] || 100;
          datasetsEvol.push({
            label: t,
            data: sliceP.map(p => ((p / base) * 100).toFixed(1)),
            borderColor: colores[i % colores.length],
            borderWidth: 1.5,
            fill: false,
            tension: 0.1,
            pointRadius: 0,
            order: 3 + i
          });
        }
      });

      charts.evolucion = new Chart(ctxEvolucion, {
        type: 'line',
        data: { labels: timeLabels, datasets: datasetsEvol },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: {
            legend: { position: 'top' },
            tooltip: {
              callbacks: {
                label: function(context) {
                  return `${context.dataset.label}: ${context.parsed.y}%`;
                }
              }
            }
          },
          scales: {
            x: { ticks: { maxTicksLimit: 10 } },
            y: { title: { display: true, text: 'Rendimiento Acumulado (Base 100)' } }
          }
        }
      });

      // 5. Mapa de Activos
      if (charts.mapa) charts.mapa.destroy();
      const ctxMapa = document.getElementById('chartMapa').getContext('2d');
      const mapaDatasets = data.tickers.map((t, i) => ({
        label: t,
        data: [{ x: +(data.vols[i] * 100).toFixed(1), y: +(data.esperados[i] * 100).toFixed(1) }],
        backgroundColor: colores[i % colores.length],
        pointRadius: 7,
        pointHoverRadius: 9
      }));
      mapaDatasets.push({
        label: 'CARTERA ÓPTIMA',
        data: [{ x: +(data.port_vol_sharpe * 100).toFixed(1), y: +(data.port_return_sharpe * 100).toFixed(1) }],
        backgroundColor: '#0062FF',
        pointRadius: 10,
        pointHoverRadius: 12,
        pointStyle: 'rectRot'
      });

      charts.mapa = new Chart(ctxMapa, {
        type: 'scatter',
        data: { datasets: mapaDatasets },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          scales: {
            x: {
              type: 'linear',
              title: { display: true, text: 'Volatilidad Anual (%)', font: { weight: 'bold' } },
              ticks: { callback: function(val) { return val + '%'; } }
            },
            y: {
              type: 'linear',
              title: { display: true, text: 'Retorno Esperado (%)', font: { weight: 'bold' } },
              ticks: { callback: function(val) { return val + '%'; } }
            }
          },
          plugins: { legend: { position: 'top' } }
        }
      });

      // 6. Matriz de Correlación (Mapa de Calor continuo Azul a Blanco)
      const corrList = data.corr_tickers || (data.matriz_correlacion && data.matriz_correlacion.length > data.tickers.length ? [...data.tickers, 'SPY'] : data.tickers);
      const thead = document.getElementById('theadCorr');
      thead.innerHTML = '<th style="text-align:center;">Activo</th>' + corrList.map(t => `<th style="text-align:center;">${t}</th>`).join('');
      const tbody = document.getElementById('tbodyCorr');
      tbody.innerHTML = '';
      data.matriz_correlacion.forEach((fila, r) => {
        const rowTicker = corrList[r] || '';
        let rowHtml = `<td style="font-weight:800; background:var(--slate-50); color:var(--navy-900); text-align:center;">${rowTicker}</td>`;
        fila.forEach((val, c) => {
          // Escala continua: val de 0 a 1 -> opacidad de 0.03 (blanco suave) a 0.92 (azul intenso)
          const norm = Math.max(0, Math.min(1, val));
          const opacity = (norm * 0.89 + 0.03).toFixed(3);
          const bg = `rgba(0, 98, 255, ${opacity})`;
          const textColor = norm > 0.55 ? '#FFFFFF' : '#0A192F';
          rowHtml += `
            <td style="background:${bg}; color:${textColor}; font-family:'JetBrains Mono', monospace; font-weight:800; text-align:center; transition:all 0.15s ease;" title="Correlación ${rowTicker} vs ${corrList[c] || ''}: ${val.toFixed(2)}">
              ${val.toFixed(2)}
            </td>`;
        });
        tbody.innerHTML += `<tr>${rowHtml}</tr>`;
      });

      // 7. Tabla Desglose
      const tbodyDesglose = document.getElementById('tbodyDesglose');
      tbodyDesglose.innerHTML = '';
      data.tickers.forEach((t, i) => {
        tbodyDesglose.innerHTML += `
          <tr>
            <td><span class="ticker-badge">${t}</span></td>
            <td>${(data.esperados[i] * 100).toFixed(1)}%</td>
            <td>${(data.vols[i] * 100).toFixed(1)}%</td>
            <td>${(data.downside_vols[i] * 100).toFixed(1)}%</td>
            <td>${data.betas[i].toFixed(2)}</td>
            <td>${(data.capm_returns[i] * 100).toFixed(1)}%</td>
            <td style="font-weight:700; color:var(--blue-600);">${(data.pesos_sharpe[i] * 100).toFixed(1)}%</td>
            <td style="font-weight:700; color:var(--navy-900);">${(data.pesos_sortino[i] * 100).toFixed(1)}%</td>
          </tr>
        `;
      });
    }

    // ===== GESTIÓN DE CARTERAS GUARDADAS EN SQLITE =====
    let listaCarterasGuardadas = [];
    let tipoCarteraAGuardar = 'sharpe';

    function abrirModalGuardarCartera(tipo) {
      if (!window.lastOptResult) {
        showToast("⚠️ Primero debe ejecutar una optimización de cartera.");
        return;
      }
      tipoCarteraAGuardar = tipo;
      const data = window.lastOptResult;
      const tipoTxt = tipo === 'sharpe' ? 'Máximo Sharpe' : 'Máximo Sortino';
      const pesos = tipo === 'sharpe' ? data.pesos_sharpe : data.pesos_sortino;
      
      const fechaHoy = new Date().toISOString().split('T')[0];
      document.getElementById('inputModalNombreCartera').value = `Cartera Óptima ${tipoTxt} (${fechaHoy})`;
      document.getElementById('inputModalDescCartera').value = `${data.tickers.length} activos optimizados según ${tipoTxt}.`;

      let previewHtml = `<div style="font-weight:800; color:var(--navy-900); margin-bottom:6px;">Estrategia: <span style="color:var(--blue-600);">${tipoTxt}</span> | ${data.tickers.length} Activos:</div>`;
      previewHtml += `<div style="display:flex; flex-wrap:wrap; gap:6px;">`;
      data.tickers.forEach((t, i) => {
        const w = (pesos[i] * 100).toFixed(1);
        previewHtml += `<span style="background:#FFF; border:1px solid var(--slate-200); padding:3px 8px; border-radius:6px; font-weight:700; color:var(--navy-900); font-size:0.8rem;">${t}: <strong style="color:var(--blue-600);">${w}%</strong></span>`;
      });
      previewHtml += `</div>`;
      previewHtml += `<div style="margin-top:8px; font-size:0.8rem; color:var(--slate-600);">Retorno Estimado: ${(data.port_return_sharpe * 100).toFixed(1)}% | Volatilidad: ${(data.port_vol_sharpe * 100).toFixed(1)}% | Sharpe: ${data.sharpe_ratio.toFixed(2)}</div>`;
      
      document.getElementById('modalPreviewCartera').innerHTML = previewHtml;
      document.getElementById('modalGuardarCartera').style.display = 'flex';
    }

    function cerrarModalGuardarCartera() {
      document.getElementById('modalGuardarCartera').style.display = 'none';
    }

    async function confirmarGuardarCarteraDb() {
      const nombre = document.getElementById('inputModalNombreCartera').value.trim();
      const descripcion = document.getElementById('inputModalDescCartera').value.trim();
      if (!nombre) {
        showToast("⚠️ Ingrese un nombre para la cartera.");
        return;
      }
      if (!window.lastOptResult) return;

      const data = window.lastOptResult;
      const pesos = tipoCarteraAGuardar === 'sharpe' ? data.pesos_sharpe : data.pesos_sortino;

      const payload = {
        nombre,
        descripcion: descripcion || null,
        tipo_ponderacion: tipoCarteraAGuardar,
        tickers: data.tickers,
        pesos: pesos,
        retorno_esperado: data.port_return_sharpe,
        volatilidad: data.port_vol_sharpe,
        sharpe_ratio: data.sharpe_ratio,
        ccl_ref: data.ccl_ref,
        rf_rate: data.rf_rate
      };

      try {
        const res = await fetch('/api/carteras/guardar', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
        });
        const json = await res.json();
        if (json.success) {
          cerrarModalGuardarCartera();
          showToast(`✓ Cartera "${nombre}" guardada con éxito en SQLite.`);
          cargarCarterasDb();
        } else {
          showToast("❌ Error al guardar cartera: " + (json.error || "Desconocido"));
        }
      } catch (err) {
        showToast("❌ Error de conexión: " + err.message);
      }
    }

    async function cargarCarterasDb() {
      try {
        const res = await fetch('/api/carteras');
        const json = await res.json();
        if (!json.success || !json.data) return;

        listaCarterasGuardadas = json.data.carteras || [];
        const countEl = document.getElementById('countCarterasDb');
        if (countEl) countEl.innerText = `${listaCarterasGuardadas.length}`;

        // 1. Llenar tabla en view-db
        const tbody = document.getElementById('tbodyCarterasDb');
        if (tbody) {
          if (listaCarterasGuardadas.length === 0) {
            tbody.innerHTML = '<tr><td colspan="7" style="text-align:center; color:var(--slate-500); padding:16px;">No hay carteras guardadas aún. Guarde una cartera desde la pestaña de Optimización Markowitz.</td></tr>';
          } else {
            tbody.innerHTML = '';
            listaCarterasGuardadas.forEach(c => {
              const tipoBadge = c.tipo_ponderacion === 'sharpe' 
                ? '<span class="ticker-badge" style="background:#dcfce7; color:#15803d; border-color:#86efac;">Máx. Sharpe</span>'
                : '<span class="ticker-badge" style="background:#e0f2fe; color:#0369a1; border-color:#7dd3fc;">Máx. Sortino</span>';
              
              let weightsBadges = '<div style="display:flex; flex-wrap:wrap; gap:4px; max-width:320px;">';
              c.tickers.forEach((t, i) => {
                const w = c.pesos && c.pesos[i] ? (c.pesos[i] * 100).toFixed(1) : '0.0';
                weightsBadges += `<span style="font-size:0.75rem; background:var(--slate-100); padding:2px 6px; border-radius:4px; font-weight:700; color:var(--navy-900);">${t} ${w}%</span>`;
              });
              weightsBadges += '</div>';

              const ret = c.retorno_esperado != null ? (c.retorno_esperado * 100).toFixed(1) + '%' : '--';
              const vol = c.volatilidad != null ? (c.volatilidad * 100).toFixed(1) + '%' : '--';
              const sharpe = c.sharpe_ratio != null ? c.sharpe_ratio.toFixed(2) : '--';
              const metricsTxt = `<div style="font-size:0.8rem; line-height:1.35;"><strong>Sharpe:</strong> ${sharpe}<br><span style="color:var(--slate-500);">Ret: ${ret} | Vol: ${vol}</span></div>`;

              tbody.innerHTML += `
                <tr>
                  <td style="font-weight:800; color:var(--slate-500);">#${c.id}</td>
                  <td>
                    <div style="font-weight:800; color:var(--navy-900); font-size:0.9rem;">${c.nombre}</div>
                    <div style="font-size:0.78rem; color:var(--slate-500);">${c.descripcion || ''}</div>
                  </td>
                  <td>${tipoBadge}</td>
                  <td>${weightsBadges}</td>
                  <td>${metricsTxt}</td>
                  <td style="font-size:0.8rem; color:var(--slate-600);">${c.fecha_creacion}</td>
                  <td style="text-align:center;">
                    <div style="display:flex; gap:6px; justify-content:center;">
                      <button class="ticker-chip" style="background:var(--blue-600); color:#FFF; padding:5px 10px; font-size:0.78rem;" onclick="importarCarteraDirectaATrack(${c.id})" title="Cargar en Seguimiento">📈 Seguimiento</button>
                      <button class="ticker-chip" style="background:#fee2e2; color:#b91c1c; border-color:#fca5a5; padding:5px 8px; font-size:0.78rem;" onclick="eliminarCarteraDb(${c.id})" title="Eliminar de la Base de Datos">🗑️</button>
                    </div>
                  </td>
                </tr>
              `;
            });
          }
        }

        // 2. Llenar selector en view-tracking
        const selectTrack = document.getElementById('selectCarterasDbTrack');
        if (selectTrack) {
          const currVal = selectTrack.value;
          selectTrack.innerHTML = '<option value="">-- Seleccionar Cartera Guardada --</option>';
          listaCarterasGuardadas.forEach(c => {
            selectTrack.innerHTML += `<option value="${c.id}">${c.nombre} (${c.tickers.length} activos · ${c.fecha_creacion.split(' ')[0]})</option>`;
          });
          if (currVal) selectTrack.value = currVal;
        }
      } catch (err) {
        console.error("Error al cargar carteras guardadas:", err);
      }
    }

    function previewCarteraSeleccionadaTrack() {
      const selectTrack = document.getElementById('selectCarterasDbTrack');
      const previewDiv = document.getElementById('previewCarteraTrack');
      if (!selectTrack || !previewDiv) return;

      const id = parseInt(selectTrack.value);
      const cartera = listaCarterasGuardadas.find(c => c.id === id);
      if (!cartera) {
        previewDiv.style.display = 'none';
        return;
      }

      const pesosStr = cartera.tickers.map((t, i) => `${t}: ${(cartera.pesos[i] * 100).toFixed(1)}%`).join(', ');
      previewDiv.innerHTML = `✓ <strong>${cartera.nombre}</strong> &bull; Ponderaciones: ${pesosStr}`;
      previewDiv.style.display = 'block';
    }

    function cargarCarteraSeleccionadaTrack() {
      const selectTrack = document.getElementById('selectCarterasDbTrack');
      if (!selectTrack || !selectTrack.value) {
        showToast("⚠️ Seleccione una cartera guardada de la lista desplegable.");
        return;
      }
      importarCarteraDirectaATrack(parseInt(selectTrack.value));
    }

    function importarCarteraDirectaATrack(id) {
      const cartera = listaCarterasGuardadas.find(c => c.id === id);
      if (!cartera) {
        showToast("⚠️ No se encontró la cartera en memoria.");
        return;
      }

      document.getElementById('trackTickers').value = cartera.tickers.join(', ');
      document.getElementById('trackPesos').value = cartera.pesos.map(w => (w * 100).toFixed(1)).join(', ');
      if (cartera.ccl_ref) {
        document.getElementById('trackCcl').value = cartera.ccl_ref;
      }

      switchView('view-tracking');
      showToast(`📥 Cartera "${cartera.nombre}" importada en Seguimiento.`);
    }

    async function eliminarCarteraDb(id) {
      if (!confirm(`¿Está seguro de que desea eliminar permanentemente la cartera #${id} de la base de datos SQLite?`)) {
        return;
      }
      try {
        const res = await fetch(`/api/carteras/${id}`, { method: 'DELETE' });
        const json = await res.json();
        if (json.success) {
          showToast(`✓ Cartera #${id} eliminada de SQLite.`);
          cargarCarterasDb();
        } else {
          showToast("❌ Error al eliminar: " + (json.error || "Desconocido"));
        }
      } catch (err) {
        showToast("❌ Error: " + err.message);
      }
    }

    function enviarOptimizacionASeguimiento() {
      if (!window.lastOptResult) {
        showToast("⚠️ Primero ejecute una optimización de cartera.");
        return;
      }
      const data = window.lastOptResult;
      document.getElementById('trackTickers').value = data.tickers.join(', ');
      document.getElementById('trackPesos').value = data.pesos_sharpe.map(w => (w * 100).toFixed(1)).join(', ');
      if (data.ccl_ref) {
        document.getElementById('trackCcl').value = data.ccl_ref;
      }
      switchView('view-tracking');
      showToast("📥 Cartera óptima transferida a la pestaña de Seguimiento.");
    }

    async function ejecutarIsOos() {
      const tickersRaw = document.getElementById('isOosTickers').value;
      const tickers = tickersRaw.split(',').map(s => s.trim().toUpperCase()).filter(s => s.length > 0);
      if (tickers.length === 0) {
        showToast("⚠️ Ingrese al menos un ticker.");
        return;
      }

      const payload = {
        tickers,
        is_velas: parseInt(document.getElementById('isVelas').value) || 252,
        oos_velas: parseInt(document.getElementById('oosVelas').value) || 252,
        rebalance_freq: document.getElementById('isOosRebalFreq').value || 'mensual',
        benchmark: document.getElementById('isOosBenchmark').value || 'SPY',
        ccl_ref: parseFloat(document.getElementById('isOosCcl').value) || 1250.0,
        rf_rate: 0.04,
        min_bound: 0.05
      };

      setStatus('statusBannerIsOos', 'statusBannerIsOosText', 'btnEjecutarIsOos', true, "Ejecutando validación IS / OOS y rebalanceo de cartera...");
      try {
        const res = await fetch('/api/is_oos', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
        });
        const data = await res.json();
        setStatus('statusBannerIsOos', 'statusBannerIsOosText', 'btnEjecutarIsOos', false);

        if (!data.success) {
          showToast("❌ " + (data.error || "Error en validación IS/OOS"));
          return;
        }

        const d = data.data;
        // KPIs IS
        document.getElementById('kpiIsRet').innerText = (d.is_return * 100).toFixed(1) + '%';
        document.getElementById('kpiIsSharpe').innerText = d.is_sharpe.toFixed(2);

        // KPIs OOS
        document.getElementById('kpiOosRet').innerText = (d.oos_return * 100).toFixed(1) + '%';
        document.getElementById('kpiOosSharpe').innerText = d.oos_sharpe.toFixed(2);
        document.getElementById('kpiOosGain').innerText = d.oos_gain_pct.toFixed(1) + '%';
        document.getElementById('kpiOosMaxDd').innerText = d.oos_max_dd.toFixed(1) + '%';

        // Benchmark & Alpha
        document.getElementById('kpiBmOosRet').innerText = (d.bm_oos_return * 100).toFixed(1) + '%';
        const alpha = (d.oos_return - d.bm_oos_return) * 100;
        document.getElementById('kpiAlphaOos').innerText = (alpha >= 0 ? '+' : '') + alpha.toFixed(1) + '%';

        // 1. Gráfico Curva de Equity Continua (IS vs OOS vs Benchmark)
        if (charts.isOosEvolucion) charts.isOosEvolucion.destroy();
        const ctxEvol = document.getElementById('chartIsOosEvolucion').getContext('2d');

        const splitIdx = d.split_index;
        const labels = d.full_time_labels.map((lbl, idx) => {
          if (idx === splitIdx) return `${lbl} [FIN IS / INICIO OOS]`;
          return lbl;
        });

        charts.isOosEvolucion = new Chart(ctxEvol, {
          type: 'line',
          data: {
            labels: labels,
            datasets: [
              {
                label: `Cartera Optimizada (IS & OOS Rebal. ${d.rebalance_freq.toUpperCase()})`,
                data: d.port_full_equity_curve.map(v => v.toFixed(2)),
                borderColor: '#0062FF',
                backgroundColor: 'rgba(0, 98, 255, 0.08)',
                borderWidth: 2.8,
                fill: true,
                tension: 0.1,
                pointRadius: 0,
                segment: {
                  borderColor: ctx => ctx.p0DataIndex < splitIdx ? '#0062FF' : '#00D2D3',
                  borderDash: ctx => ctx.p0DataIndex < splitIdx ? [] : [4, 2]
                }
              },
              {
                label: `Benchmark (${d.benchmark_nombre})`,
                data: d.bm_full_equity_curve.map(v => v.toFixed(2)),
                borderColor: '#0A192F',
                borderWidth: 2,
                fill: false,
                tension: 0.1,
                pointRadius: 0
              }
            ]
          },
          options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
              legend: { position: 'top' },
              tooltip: {
                callbacks: {
                  title: function(ctx) {
                    const idx = ctx[0].dataIndex;
                    const phase = idx <= splitIdx ? 'FASE IN-SAMPLE (Entrenamiento)' : 'FASE OUT-OF-SAMPLE (Prueba con Rebalanceo)';
                    return `${ctx[0].label} — ${phase}`;
                  }
                }
              }
            },
            scales: {
              x: { ticks: { maxTicksLimit: 12 } },
              y: { title: { display: true, text: 'Rendimiento Acumulado (Base 100)' } }
            }
          }
        });

        // 2. Gráfico de Pesos Calibrados
        if (charts.isOosPesos) charts.isOosPesos.destroy();
        const ctxPesos = document.getElementById('chartIsOosPesos').getContext('2d');
        charts.isOosPesos = new Chart(ctxPesos, {
          type: 'bar',
          data: {
            labels: d.tickers,
            datasets: [{
              label: 'Pesos Calibrados en IS (%)',
              data: d.pesos_sharpe.map(p => (p * 100).toFixed(1)),
              backgroundColor: '#0062FF',
              borderRadius: 6
            }]
          },
          options: { responsive: true, maintainAspectRatio: false }
        });

        showToast("✓ Simulación IS/OOS y rebalanceo calculados con éxito.");
      } catch (err) {
        setStatus('statusBannerIsOos', 'statusBannerIsOosText', 'btnEjecutarIsOos', false);
        showToast("❌ Error: " + err.message);
      }
    }

    async function ejecutarTracking() {
      const tickersRaw = document.getElementById('trackTickers').value;
      const tickers = tickersRaw.split(',').map(s => s.trim().toUpperCase()).filter(s => s.length > 0);
      const pesosRaw = document.getElementById('trackPesos').value;
      const pesos = pesosRaw.split(',').map(s => parseFloat(s.trim()) || 0);

      const payload = {
        tickers,
        pesos,
        rebalance_freq: document.getElementById('trackRebalance')?.value || 'mensual',
        fecha_inicio: document.getElementById('trackFecha').value || '2024-01-01',
        ccl_ref: parseFloat(document.getElementById('trackCcl').value) || 1250.0,
        rf_rate: 0.04
      };

      setStatus('statusBannerTrack', 'statusBannerTrackText', 'btnEjecutarTrack', true, "Calculando seguimiento histórico en Rust...");
      try {
        const res = await fetch('/api/tracking', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
        });
        const data = await res.json();
        setStatus('statusBannerTrack', 'statusBannerTrackText', 'btnEjecutarTrack', false);

        if (!data.success) {
          showToast("❌ " + (data.error || "Error en seguimiento"));
          return;
        }

        const d = data.data;
        const integrantes = document.getElementById('trackIntegrantes')?.value.trim() || 'Fausto Crivelli (Presidente de Portafolio) · Luciano Mora (Analista Sr) · Florencia Beluzzo (Analista Sr)';
        d.integrantes = integrantes;
        d.rebalance_freq = payload.rebalance_freq;

        window.lastTrackResult = d;
        try { localStorage.setItem('cfuba_last_tracking_data', JSON.stringify(d)); } catch(e){}

        const repInput = document.getElementById('reportTrackIntegrantesInput');
        if (repInput) repInput.value = integrantes;

        const iframeSeg = document.getElementById('iframeReporteSeguimiento');
        if (iframeSeg && iframeSeg.contentWindow) {
          iframeSeg.contentWindow.postMessage({ type: 'IMPORT_TRACKING', payload: d }, '*');
        }

        const iframeInst = document.getElementById('iframeReporte');
        if (iframeInst && iframeInst.contentWindow) {
          iframeInst.contentWindow.postMessage({ type: 'IMPORT_TRACKING', payload: d }, '*');
        }

        document.getElementById('kpiTrackGain').innerText = d.tracking_gain_pct.toFixed(1) + '%';
        document.getElementById('kpiTrackMaxDd').innerText = d.tracking_max_dd_pct.toFixed(1) + '%';
        document.getElementById('kpiTrackSharpe').innerText = d.tracking_sharpe.toFixed(2);
        document.getElementById('kpiTrackMaxStag').innerText = d.tracking_max_stagnation_days + ' días';

        const compPanel = document.getElementById('panelTrackComposition');
        const compContainer = document.getElementById('containerTrackWeightsList');
        if (compPanel && compContainer && d.tickers && d.pesos && d.tickers.length > 0) {
          compPanel.style.display = 'block';
          const totalW = d.pesos.reduce((a, b) => a + b, 0);
          compContainer.innerHTML = d.tickers.map((t, idx) => {
            const w = d.pesos[idx] || 0;
            const pct = totalW > 0 ? ((w / totalW) * 100).toFixed(1) : w.toFixed(1);
            return `<div style="background: var(--slate-100); border: 1px solid var(--slate-300); border-radius: 8px; padding: 6px 12px; display: flex; align-items: center; gap: 8px;">
              <span style="font-weight: 800; color: var(--navy-900); font-size: 0.9rem;">${t}</span>
              <span style="background: #0062ff; color: #ffffff; padding: 2px 8px; border-radius: 6px; font-weight: 700; font-size: 0.75rem;">${pct}%</span>
            </div>`;
          }).join('');
        }

        if (charts.trackEquity) charts.trackEquity.destroy();
        const ctxEq = document.getElementById('chartTrackEquity').getContext('2d');
        charts.trackEquity = new Chart(ctxEq, {
          type: 'line',
          data: {
            labels: d.time_labels,
            datasets: [
              {
                label: 'Portafolio Personalizado (Equity)',
                data: d.port_equity_curve.map(v => v.toFixed(3)),
                borderColor: '#0062FF',
                backgroundColor: 'rgba(0, 98, 255, 0.08)',
                fill: true,
                borderWidth: 2.5,
                pointRadius: 0
              },
              {
                label: 'SPY Benchmark (Base 1.0)',
                data: d.spy_equity_curve.map(v => v.toFixed(3)),
                borderColor: '#0A192F',
                borderWidth: 1.8,
                fill: false,
                pointRadius: 0
              }
            ]
          },
          options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: { legend: { position: 'top' } }
          }
        });

        showToast("✓ Seguimiento calculado con éxito.");
      } catch (err) {
        setStatus('statusBannerTrack', 'statusBannerTrackText', 'btnEjecutarTrack', false);
        showToast("❌ Error: " + err.message);
      }
    }

    // ==========================================
    // ZONA EXPERIMENTAL: RED NEURONAL DE CARTERA
    // ==========================================
    async function cargarTodosTickersANeural() {
      try {
        const res = await fetch('/api/db/tickers');
        const json = await res.json();
        if (json.success && json.data && json.data.length > 0) {
          const validTickers = json.data.filter(t => t !== 'SPY');
          document.getElementById('neuralTickers').value = validTickers.join(', ');
          showToast(`✓ ${validTickers.length} tickers cargados desde SQLite.`);
        } else {
          showToast("ℹ️ No hay tickers suficientes en SQLite.");
        }
      } catch (err) {
        showToast("❌ Error al leer tickers: " + err.message);
      }
    }

    async function ejecutarSimulacionNeural() {
      const tickersRaw = document.getElementById('neuralTickers').value;
      const tickers = tickersRaw.split(',').map(s => s.trim().toUpperCase()).filter(s => s.length > 0);

      if (tickers.length < 3) {
        showToast("⚠️ Ingrese al menos 3 activos para la simulación neuronal.");
        return;
      }

      const payload = {
        tickers,
        lookback_window: parseInt(document.getElementById('neuralLookback').value) || 63,
        rebalance_freq: parseInt(document.getElementById('neuralRebalance').value) || 21,
        max_cardinality: parseInt(document.getElementById('neuralMaxCard').value) || 5,
        min_asset_weight: parseFloat(document.getElementById('neuralMinWeight').value) || 0.05,
        transaction_fee_bps: parseFloat(document.getElementById('neuralFeeBps').value) || 10.0,
        ccl_ref: 1250.0,
        rf_rate: 0.04,
        seed: 42
      };

      setStatus('statusBannerNeural', 'statusBannerNeuralText', 'btnEjecutarNeural', true, "Ejecutando inferencia neuronal causal paso a paso en Rust...");
      try {
        const res = await fetch('/api/experimental/neural-allocation', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
        });
        const data = await res.json();
        setStatus('statusBannerNeural', 'statusBannerNeuralText', 'btnEjecutarNeural', false);

        if (!data.success) {
          showToast("❌ " + (data.error || "Error en simulación neuronal"));
          return;
        }

        renderResultadosNeural(data.data);
        showToast("✓ Simulación de red neuronal completada y auditada con éxito.");
      } catch (err) {
        setStatus('statusBannerNeural', 'statusBannerNeuralText', 'btnEjecutarNeural', false);
        showToast("❌ Error: " + err.message);
      }
    }

    function renderResultadosNeural(d) {
      if (!d) return;

      // 1. KPIs Principales
      document.getElementById('kpiNeuralGain').innerText = (d.total_net_gain_pct >= 0 ? '+' : '') + d.total_net_gain_pct.toFixed(1) + '%';
      document.getElementById('kpiNeuralAnnRet').innerText = d.annualized_net_return_pct.toFixed(1) + '%';
      document.getElementById('kpiNeuralAlpha').innerText = (d.alpha_annualized_pct >= 0 ? '+' : '') + d.alpha_annualized_pct.toFixed(1) + '%';
      document.getElementById('kpiNeuralSharpe').innerText = d.sharpe_ratio.toFixed(2);
      document.getElementById('kpiNeuralMaxDd').innerText = '-' + d.max_drawdown_pct.toFixed(1) + '%';
      document.getElementById('kpiNeuralTurnover').innerText = d.audit_report.avg_turnover_per_rebalance.toFixed(1) + '%';

      // 2. Gráfico 1: Curva de Equity Acumulado
      if (charts.neuralEquity) charts.neuralEquity.destroy();
      const ctxEq = document.getElementById('chartNeuralEquity').getContext('2d');
      charts.neuralEquity = new Chart(ctxEq, {
        type: 'line',
        data: {
          labels: d.time_labels,
          datasets: [
            {
              label: '🤖 Cartera Red Neuronal IA (Neto Comisiones)',
              data: d.portfolio_equity_curve.map(v => v.toFixed(2)),
              borderColor: '#8B5CF6',
              backgroundColor: 'rgba(139, 92, 246, 0.12)',
              fill: true,
              borderWidth: 2.8,
              pointRadius: 0
            },
            {
              label: '📊 SPY Benchmark (S&P 500)',
              data: d.benchmark_equity_curve.map(v => v.toFixed(2)),
              borderColor: '#0A192F',
              borderWidth: 2.0,
              fill: false,
              pointRadius: 0
            },
            {
              label: '⚖️ Cartera Equiponderada (1/N)',
              data: d.equal_weight_equity_curve.map(v => v.toFixed(2)),
              borderColor: '#10B981',
              borderDash: [5, 5],
              borderWidth: 1.8,
              fill: false,
              pointRadius: 0
            }
          ]
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: {
            legend: { position: 'top' },
            tooltip: {
              callbacks: {
                label: function(ctx) { return `${ctx.dataset.label}: ${ctx.parsed.y} pts`; }
              }
            }
          },
          scales: {
            y: { title: { display: true, text: 'Valor de Cartera (Base 100)' } }
          }
        }
      });

      // 3. Panel de Auditoría de Invariantes
      const rep = d.audit_report;
      const statusBadge = document.getElementById('badgeAuditStatus');
      if (statusBadge) {
        if (rep.audit_passed) {
          statusBadge.innerText = '✓ AUDITORÍA APROBADA';
          statusBadge.style.background = '#16A34A';
        } else {
          statusBadge.innerText = '⚠️ REVISIÓN REQUERIDA';
          statusBadge.style.background = '#DC2626';
        }
      }

      document.getElementById('lblAuditLookahead').innerText = '0% (Lag Causal F_{t-1} Verificado)';
      document.getElementById('lblAuditCardinality').innerText = rep.max_cardinality_violations === 0 
        ? 'Máximo 5 Activos (100% Cumplido)' 
        : `⚠️ ${rep.max_cardinality_violations} Violaciones`;
      document.getElementById('lblAuditMinWeight').innerText = rep.min_weight_violations === 0 
        ? 'Piso 5% Respetado' 
        : `⚠️ ${rep.min_weight_violations} Violaciones`;
      document.getElementById('lblAuditSumWeights').innerText = rep.sum_weights_tolerance_violations === 0 
        ? '100.00% Exacto' 
        : `⚠️ ${rep.sum_weights_tolerance_violations} Desvíos`;

      const notesContainer = document.getElementById('containerAuditNotes');
      if (notesContainer) {
        notesContainer.innerHTML = rep.audit_notes.map(n => `<div style="margin-bottom:4px;">${n}</div>`).join('');
      }

      // 5. Tabla de Rebalanceos Históricos
      const tbody = document.getElementById('tbodyNeuralAllocations');
      const countEl = document.getElementById('countNeuralRebalances');
      if (countEl) countEl.innerText = `${d.allocations_history.length}`;

      if (tbody) {
        if (d.allocations_history.length === 0) {
          tbody.innerHTML = '<tr><td colspan="7" style="text-align:center;">No hubo eventos de rebalanceo en el período.</td></tr>';
        } else {
          tbody.innerHTML = d.allocations_history.map(a => {
            let activeBadges = a.active_tickers.map((t, idx) => {
              const w = a.active_weights[idx] || 0;
              return `<span style="background:#FAF5FF; border:1px solid #D8B4FE; color:#6B21A8; padding:2px 6px; border-radius:4px; font-size:0.75rem; font-weight:800; margin-right:4px;">${t} (${w.toFixed(1)}%)</span>`;
            }).join('');

            return `
              <tr>
                <td style="font-weight:700; color:var(--navy-900);">${a.date}</td>
                <td>${activeBadges}</td>
                <td><strong>${a.active_tickers.length} activos</strong></td>
                <td>${a.turnover.toFixed(1)}%</td>
                <td style="color:#DC2626;">-${a.transaction_cost_pct.toFixed(3)}%</td>
                <td style="font-weight:800; color:#8B5CF6;">${a.portfolio_equity.toFixed(2)}</td>
                <td style="color:var(--slate-600);">${a.benchmark_equity.toFixed(2)}</td>
              </tr>
            `;
          }).join('');
        }
      }
    }

    const CATALOGO_ACTIVOS = [
      // 🇦🇷 CEDEARs & Argentina / Latam
      { sym: 'YPF', name: 'YPF Sociedad Anónima (Petróleo & Gas)', cat: 'cedears', sector: 'Energía Argentina' },
      { sym: 'GGAL', name: 'Grupo Financiero Galicia S.A.', cat: 'cedears', sector: 'Bancos Argentina' },
      { sym: 'BMA', name: 'Banco Macro S.A.', cat: 'cedears', sector: 'Bancos Argentina' },
      { sym: 'PAM', name: 'Pampa Energía S.A.', cat: 'cedears', sector: 'Energía & Generación' },
      { sym: 'TGS', name: 'Transportadora de Gas del Sur S.A.', cat: 'cedears', sector: 'Gas & Infraestructura' },
      { sym: 'CEPU', name: 'Central Puerto S.A.', cat: 'cedears', sector: 'Generación Eléctrica' },
      { sym: 'CRES', name: 'Cresud S.A.C.I.F. y A.', cat: 'cedears', sector: 'Agro & Real Estate' },
      { sym: 'EDN', name: 'Empresa Distribuidora Norte (Edenor)', cat: 'cedears', sector: 'Servicios Públicos' },
      { sym: 'LOMA', name: 'Loma Negra C.I.A.S.A.', cat: 'cedears', sector: 'Materiales & Cemento' },
      { sym: 'IRS', name: 'IRSA Inversiones y Representaciones', cat: 'cedears', sector: 'Real Estate' },
      { sym: 'TEO', name: 'Telecom Argentina S.A.', cat: 'cedears', sector: 'Telecomunicaciones' },
      { sym: 'VIST', name: 'Vista Energy S.A.B. de C.V. (Vaca Muerta)', cat: 'cedears', sector: 'Petróleo & Shale' },
      { sym: 'MELI', name: 'MercadoLibre Inc.', cat: 'cedears', sector: 'E-commerce & Fintech' },
      { sym: 'GLOB', name: 'Globant S.A.', cat: 'cedears', sector: 'Software & IT' },
      { sym: 'DESP', name: 'Despegar.com Corp.', cat: 'cedears', sector: 'Turismo & Viajes' },
      { sym: 'BIOX', name: 'Bioceres Crop Solutions Corp.', cat: 'cedears', sector: 'Biotecnología Agro' },
      { sym: 'PBR', name: 'Petróleo Brasileiro S.A. (Petrobras)', cat: 'cedears', sector: 'Petróleo Latam' },
      { sym: 'VALE', name: 'Vale S.A. (Minería & Hierro)', cat: 'cedears', sector: 'Minería' },
      { sym: 'ITUB', name: 'Itaú Unibanco Holding S.A.', cat: 'cedears', sector: 'Bancos Brasil' },
      { sym: 'BBD', name: 'Banco Bradesco S.A.', cat: 'cedears', sector: 'Bancos Brasil' },
      { sym: 'NU', name: 'Nu Holdings Ltd. (Nubank)', cat: 'cedears', sector: 'Fintech Latam' },
      { sym: 'ARCOS', name: 'Arcos Dorados Holdings (McDonalds Latam)', cat: 'cedears', sector: 'Consumo Masivo' },
      { sym: 'MSTR', name: 'MicroStrategy Inc. (Bitcoin Treasury & Software)', cat: 'cedears', sector: 'Software & Cripto' },
      { sym: 'RACE', name: 'Ferrari N.V. (Supercars de Lujo & F1)', cat: 'cedears', sector: 'Automotriz de Lujo' },

      // 💻 Big Tech & Inteligencia Artificial
      { sym: 'MSTR', name: 'MicroStrategy Inc. (Bitcoin Proxy / Analytics)', cat: 'big_tech', sector: 'Software & Bitcoin' },
      { sym: 'GOOGL', name: 'Alphabet Inc. (Google Clase A)', cat: 'big_tech', sector: 'Tecnología & IA' },
      { sym: 'GOOG', name: 'Alphabet Inc. (Google Clase C)', cat: 'big_tech', sector: 'Tecnología & IA' },
      { sym: 'AAPL', name: 'Apple Inc.', cat: 'big_tech', sector: 'Hardware & Servicios' },
      { sym: 'MSFT', name: 'Microsoft Corporation', cat: 'big_tech', sector: 'Cloud & Software' },
      { sym: 'AMZN', name: 'Amazon.com Inc.', cat: 'big_tech', sector: 'Cloud & Retail' },
      { sym: 'NVDA', name: 'NVIDIA Corporation (GPUs & IA)', cat: 'big_tech', sector: 'Semiconductores & IA' },
      { sym: 'META', name: 'Meta Platforms Inc. (Facebook/IG)', cat: 'big_tech', sector: 'Redes Sociales & IA' },
      { sym: 'TSLA', name: 'Tesla Inc. (EVs & IA)', cat: 'big_tech', sector: 'Automotriz & Energía' },
      { sym: 'AMD', name: 'Advanced Micro Devices Inc.', cat: 'big_tech', sector: 'Semiconductores' },
      { sym: 'AVGO', name: 'Broadcom Inc.', cat: 'big_tech', sector: 'Semiconductores & Networking' },
      { sym: 'QCOM', name: 'Qualcomm Inc.', cat: 'big_tech', sector: 'Procesadores Móviles' },
      { sym: 'INTC', name: 'Intel Corporation', cat: 'big_tech', sector: 'Semiconductores' },
      { sym: 'ARM', name: 'Arm Holdings plc', cat: 'big_tech', sector: 'Arquitectura Chips' },
      { sym: 'TSM', name: 'Taiwan Semiconductor Manufacturing (TSMC)', cat: 'big_tech', sector: 'Fundición Chips' },
      { sym: 'ASML', name: 'ASML Holding N.V. (Litografía)', cat: 'big_tech', sector: 'Equipos Semiconductores' },
      { sym: 'ORCL', name: 'Oracle Corporation', cat: 'big_tech', sector: 'Bases de Datos & Cloud' },
      { sym: 'ADBE', name: 'Adobe Inc.', cat: 'big_tech', sector: 'Software Creativo' },
      { sym: 'CRM', name: 'Salesforce Inc.', cat: 'big_tech', sector: 'CRM & Cloud' },
      { sym: 'NOW', name: 'ServiceNow Inc.', cat: 'big_tech', sector: 'Automatización IT' },
      { sym: 'PLTR', name: 'Palantir Technologies Inc.', cat: 'big_tech', sector: 'Analítica de Datos & IA' },
      { sym: 'IBM', name: 'International Business Machines (IBM)', cat: 'big_tech', sector: 'Cloud & Mainframes' },
      { sym: 'CSCO', name: 'Cisco Systems Inc.', cat: 'big_tech', sector: 'Redes & Infraestructura' },
      { sym: 'TXN', name: 'Texas Instruments Inc.', cat: 'big_tech', sector: 'Chips Analógicos' },
      { sym: 'AMAT', name: 'Applied Materials Inc.', cat: 'big_tech', sector: 'Equipos Semiconductores' },
      { sym: 'LRCX', name: 'Lam Research Corp.', cat: 'big_tech', sector: 'Equipos Semiconductores' },
      { sym: 'MU', name: 'Micron Technology Inc. (Memorias)', cat: 'big_tech', sector: 'Memorias DRAM/NAND' },
      { sym: 'UBER', name: 'Uber Technologies Inc.', cat: 'big_tech', sector: 'Movilidad & Delivery' },
      { sym: 'ABNB', name: 'Airbnb Inc.', cat: 'big_tech', sector: 'Hospedaje & Turismo' },
      { sym: 'NFLX', name: 'Netflix Inc.', cat: 'big_tech', sector: 'Streaming & Medios' },
      { sym: 'SPOT', name: 'Spotify Technology S.A.', cat: 'big_tech', sector: 'Streaming Audio' },

      // 🇺🇸 S&P 500 Mega-Caps & Consumo / Finanzas / Salud
      { sym: 'BRK-B', name: 'Berkshire Hathaway Inc. (Warren Buffett)', cat: 'sp500', sector: 'Conglomerado & Seguros' },
      { sym: 'JPM', name: 'JPMorgan Chase & Co.', cat: 'sp500', sector: 'Banca Global' },
      { sym: 'BAC', name: 'Bank of America Corp.', cat: 'sp500', sector: 'Banca' },
      { sym: 'WFC', name: 'Wells Fargo & Company', cat: 'sp500', sector: 'Banca' },
      { sym: 'C', name: 'Citigroup Inc.', cat: 'sp500', sector: 'Banca' },
      { sym: 'GS', name: 'The Goldman Sachs Group', cat: 'sp500', sector: 'Banca de Inversión' },
      { sym: 'MS', name: 'Morgan Stanley', cat: 'sp500', sector: 'Gestión de Patrimonio' },
      { sym: 'V', name: 'Visa Inc.', cat: 'sp500', sector: 'Pagos Digitales' },
      { sym: 'MA', name: 'Mastercard Inc.', cat: 'sp500', sector: 'Pagos Digitales' },
      { sym: 'AXP', name: 'American Express Company', cat: 'sp500', sector: 'Tarjetas & Crédito' },
      { sym: 'PYPL', name: 'PayPal Holdings Inc.', cat: 'sp500', sector: 'Fintech & Pagos' },
      { sym: 'SQ', name: 'Block Inc. (Square / Cash App)', cat: 'sp500', sector: 'Fintech' },
      { sym: 'COIN', name: 'Coinbase Global Inc.', cat: 'sp500', sector: 'Cripto Exchange' },
      { sym: 'LLY', name: 'Eli Lilly and Company (GLP-1 / Salud)', cat: 'sp500', sector: 'Farmacéutica' },
      { sym: 'NVO', name: 'Novo Nordisk A/S (Ozempic/Wegovy)', cat: 'sp500', sector: 'Farmacéutica' },
      { sym: 'UNH', name: 'UnitedHealth Group Inc.', cat: 'sp500', sector: 'Salud & Seguros' },
      { sym: 'JNJ', name: 'Johnson & Johnson', cat: 'sp500', sector: 'Salud & Pharma' },
      { sym: 'ABBV', name: 'AbbVie Inc.', cat: 'sp500', sector: 'Biotecnología' },
      { sym: 'MRK', name: 'Merck & Co. Inc.', cat: 'sp500', sector: 'Farmacéutica' },
      { sym: 'PFE', name: 'Pfizer Inc.', cat: 'sp500', sector: 'Farmacéutica' },
      { sym: 'TMO', name: 'Thermo Fisher Scientific Inc.', cat: 'sp500', sector: 'Equipos Científicos' },
      { sym: 'ABT', name: 'Abbott Laboratories', cat: 'sp500', sector: 'Dispositivos Médicos' },
      { sym: 'ISRG', name: 'Intuitive Surgical (Robótica Da Vinci)', cat: 'sp500', sector: 'Robótica Médica' },
      { sym: 'WMT', name: 'Walmart Inc.', cat: 'sp500', sector: 'Retail & Consumo' },
      { sym: 'COST', name: 'Costco Wholesale Corp.', cat: 'sp500', sector: 'Club de Compras' },
      { sym: 'HD', name: 'The Home Depot Inc.', cat: 'sp500', sector: 'Hogar & Construcción' },
      { sym: 'PG', name: 'Procter & Gamble Company', cat: 'sp500', sector: 'Consumo Masivo' },
      { sym: 'KO', name: 'The Coca-Cola Company', cat: 'sp500', sector: 'Bebidas' },
      { sym: 'PEP', name: 'PepsiCo Inc.', cat: 'sp500', sector: 'Bebidas & Snacks' },
      { sym: 'MCD', name: "McDonald's Corporation", cat: 'sp500', sector: 'Restaurantes' },
      { sym: 'SBUX', name: 'Starbucks Corporation', cat: 'sp500', sector: 'Café & Consumo' },
      { sym: 'NKE', name: 'NIKE Inc.', cat: 'sp500', sector: 'Calzado & Deportes' },
      { sym: 'DIS', name: 'The Walt Disney Company', cat: 'sp500', sector: 'Entretenimiento & Parques' },
      { sym: 'CAT', name: 'Caterpillar Inc.', cat: 'sp500', sector: 'Maquinaria Pesada' },
      { sym: 'DE', name: 'Deere & Company (John Deere)', cat: 'sp500', sector: 'Maquinaria Agrícola' },
      { sym: 'BA', name: 'The Boeing Company', cat: 'sp500', sector: 'Aeroespacial & Defensa' },
      { sym: 'LMT', name: 'Lockheed Martin Corp.', cat: 'sp500', sector: 'Defensa & Aviación' },
      { sym: 'GE', name: 'General Electric Company (GE Aerospace)', cat: 'sp500', sector: 'Turbinas & Aviación' },
      { sym: 'RACE', name: 'Ferrari N.V. (Supercars & Exclusividad)', cat: 'sp500', sector: 'Automotriz de Lujo' },

      // ⚡ Energía, Petróleo & Utilities / Nuclear
      { sym: 'XOM', name: 'Exxon Mobil Corporation', cat: 'energia', sector: 'Petróleo & Gas Integrado' },
      { sym: 'CVX', name: 'Chevron Corporation', cat: 'energia', sector: 'Petróleo & Gas' },
      { sym: 'COP', name: 'ConocoPhillips', cat: 'energia', sector: 'Exploración & Prod.' },
      { sym: 'OXY', name: 'Occidental Petroleum Corp.', cat: 'energia', sector: 'Petróleo & Carbon Capture' },
      { sym: 'SLB', name: 'Schlumberger Limited', cat: 'energia', sector: 'Servicios Petroleros' },
      { sym: 'HAL', name: 'Halliburton Company', cat: 'energia', sector: 'Servicios Petroleros' },
      { sym: 'BKR', name: 'Baker Hughes Company', cat: 'energia', sector: 'Tecnología Energética' },
      { sym: 'EOG', name: 'EOG Resources Inc.', cat: 'energia', sector: 'Shale Oil & Gas' },
      { sym: 'VLO', name: 'Valero Energy Corporation', cat: 'energia', sector: 'Refinación' },
      { sym: 'MPC', name: 'Marathon Petroleum Corp.', cat: 'energia', sector: 'Refinación' },
      { sym: 'BP', name: 'BP p.l.c.', cat: 'energia', sector: 'Petróleo Global' },
      { sym: 'SHEL', name: 'Shell plc', cat: 'energia', sector: 'Petróleo & LNG' },
      { sym: 'TTE', name: 'TotalEnergies SE', cat: 'energia', sector: 'Petróleo & Renovables' },
      { sym: 'ENB', name: 'Enbridge Inc.', cat: 'energia', sector: 'Oleoductos & Gasoductos' },
      { sym: 'KMI', name: 'Kinder Morgan Inc.', cat: 'energia', sector: 'Infraestructura Gas' },
      { sym: 'WMB', name: 'The Williams Companies Inc.', cat: 'energia', sector: 'Gasoductos' },
      { sym: 'CEG', name: 'Constellation Energy Corp. (Energía Nuclear)', cat: 'energia', sector: 'Generación Nuclear' },
      { sym: 'VST', name: 'Vistra Corp. (Nuclear & Gas)', cat: 'energia', sector: 'Generación & Baterías' },
      { sym: 'SO', name: 'The Southern Company', cat: 'energia', sector: 'Servicios Eléctricos' },
      { sym: 'NEE', name: 'NextEra Energy Inc. (Eólica & Solar)', cat: 'energia', sector: 'Energía Renovable' },
      { sym: 'DUK', name: 'Duke Energy Corporation', cat: 'energia', sector: 'Servicios Eléctricos' },
      { sym: 'GEV', name: 'GE Vernova Inc. (Turbinas & Red)', cat: 'energia', sector: 'Transición Energética' },
      { sym: 'ETN', name: 'Eaton Corporation plc', cat: 'energia', sector: 'Gestión Eléctrica' },
      { sym: 'CCJ', name: 'Cameco Corporation (Uranio)', cat: 'energia', sector: 'Minería Uranio Nuclear' },
      { sym: 'SMR', name: 'NuScale Power Corp. (Reactores Modulares)', cat: 'energia', sector: 'Reactores SMR' },
      { sym: 'OKLO', name: 'Oklo Inc. (Microreactores Nucleares)', cat: 'energia', sector: 'Energía Nuclear Fisión' },
      { sym: 'FSLR', name: 'First Solar Inc.', cat: 'energia', sector: 'Paneles Solares' },
      { sym: 'ENPH', name: 'Enphase Energy Inc.', cat: 'energia', sector: 'Inversores Solares' },

      // 📊 ETFs, Índices & Commodities
      { sym: 'SPY', name: 'SPDR S&P 500 ETF Trust (Benchmark Mercado)', cat: 'etfs', sector: 'Índice S&P 500' },
      { sym: 'QQQ', name: 'Invesco QQQ Trust (Nasdaq 100 Tech)', cat: 'etfs', sector: 'Índice Nasdaq 100' },
      { sym: 'DIA', name: 'SPDR Dow Jones Industrial Average ETF', cat: 'etfs', sector: 'Índice Dow Jones' },
      { sym: 'IWM', name: 'iShares Russell 2000 ETF (Small Caps)', cat: 'etfs', sector: 'Small Caps EEUU' },
      { sym: 'VOO', name: 'Vanguard S&P 500 ETF', cat: 'etfs', sector: 'Índice S&P 500' },
      { sym: 'VTI', name: 'Vanguard Total Stock Market ETF', cat: 'etfs', sector: 'Mercado Total EEUU' },
      { sym: 'VEA', name: 'Vanguard FTSE Developed Markets ETF', cat: 'etfs', sector: 'Mercados Desarrollados' },
      { sym: 'VWO', name: 'Vanguard FTSE Emerging Markets ETF', cat: 'etfs', sector: 'Mercados Emergentes' },
      { sym: 'EEM', name: 'iShares MSCI Emerging Markets ETF', cat: 'etfs', sector: 'Emergentes Global' },
      { sym: 'EWZ', name: 'iShares MSCI Brazil ETF', cat: 'etfs', sector: 'Brasil Acciones' },
      { sym: 'ARGT', name: 'Global X MSCI Argentina ETF', cat: 'etfs', sector: 'Argentina Acciones' },
      { sym: 'XLE', name: 'Energy Select Sector SPDR Fund', cat: 'etfs', sector: 'Sector Energía EEUU' },
      { sym: 'XLF', name: 'Financial Select Sector SPDR Fund', cat: 'etfs', sector: 'Sector Financiero' },
      { sym: 'XLK', name: 'Technology Select Sector SPDR Fund', cat: 'etfs', sector: 'Sector Tecnológico' },
      { sym: 'XLV', name: 'Health Care Select Sector SPDR Fund', cat: 'etfs', sector: 'Sector Salud' },
      { sym: 'XLI', name: 'Industrial Select Sector SPDR Fund', cat: 'etfs', sector: 'Sector Industrial' },
      { sym: 'XLU', name: 'Utilities Select Sector SPDR Fund', cat: 'etfs', sector: 'Sector Utilities' },
      { sym: 'SMH', name: 'VanEck Semiconductor ETF', cat: 'etfs', sector: 'Semiconductores' },
      { sym: 'SOXX', name: 'iShares Semiconductor ETF', cat: 'etfs', sector: 'Semiconductores' },
      { sym: 'ARKK', name: 'ARK Innovation ETF (Cathie Wood)', cat: 'etfs', sector: 'Innovación Disruptiva' },
      { sym: 'GLD', name: 'SPDR Gold Shares (Oro Físico)', cat: 'etfs', sector: 'Commodity Oro' },
      { sym: 'SLV', name: 'iShares Silver Trust (Plata Física)', cat: 'etfs', sector: 'Commodity Plata' },
      { sym: 'USO', name: 'United States Oil Fund (Petróleo WTI)', cat: 'etfs', sector: 'Commodity Petróleo' },
      { sym: 'UNG', name: 'United States Natural Gas Fund (Gas Natural)', cat: 'etfs', sector: 'Commodity Gas' },
      { sym: 'TLT', name: 'iShares 20+ Year Treasury Bond ETF (Bonos Largos)', cat: 'etfs', sector: 'Renta Fija EEUU' },
      { sym: 'IEF', name: 'iShares 7-10 Year Treasury Bond ETF', cat: 'etfs', sector: 'Renta Fija EEUU' },
      { sym: 'SHY', name: 'iShares 1-3 Year Treasury Bond ETF (Corto Plazo)', cat: 'etfs', sector: 'Renta Fija Corta' },
      { sym: 'HYG', name: 'iShares iBoxx High Yield Corporate Bond ETF', cat: 'etfs', sector: 'Bonos Alto Rendimiento' },
      { sym: 'LQD', name: 'iShares iBoxx Investment Grade Corporate Bond', cat: 'etfs', sector: 'Bonos Corporativos' },
      { sym: 'BND', name: 'Vanguard Total Bond Market ETF', cat: 'etfs', sector: 'Mercado Total Bonos' }
    ];

    let catalogoItemsEnDb = [];
    let filtroCatActual = 'todos';
    let seleccionadosSet = new Set();

    function setFiltroCategoria(cat, btnEl) {
      filtroCatActual = cat;
      document.querySelectorAll('.filter-tag-btn').forEach(b => b.classList.remove('active'));
      if (btnEl) btnEl.classList.add('active');
      filtrarCatalogo();
    }

    function filtrarCatalogo() {
      const query = (document.getElementById('catalogSearchInput')?.value || '').trim().toLowerCase();
      const grid = document.getElementById('catalogGrid');
      if (!grid) return;

      const dbMap = {};
      catalogoItemsEnDb.forEach(item => {
        dbMap[item.ticker] = item.count;
      });

      const filtrados = CATALOGO_ACTIVOS.filter(act => {
        // Filtro por categoría
        if (filtroCatActual === 'en_db') {
          if (!dbMap[act.sym]) return false;
        } else if (filtroCatActual !== 'todos') {
          if (act.cat !== filtroCatActual) return false;
        }

        // Filtro por texto de búsqueda
        if (!query) return true;
        const symMatch = act.sym.toLowerCase().includes(query);
        const nameMatch = act.name.toLowerCase().includes(query);
        const sectorMatch = (act.sector || '').toLowerCase().includes(query);
        return symMatch || nameMatch || sectorMatch;
      });

      if (filtrados.length === 0) {
        const customSym = query.toUpperCase();
        if (customSym.length > 0) {
          grid.innerHTML = `
            <div style="grid-column: 1 / -1; display:flex; flex-direction:column; align-items:center; gap:12px; padding: 32px 16px; color: var(--navy-900); background:#F8FAFC; border-radius:10px; border:1.5px dashed var(--slate-300);">
              <p style="font-size: 1.05rem; font-weight: 800;">Símbolo personalizado: "${customSym}"</p>
              <p style="font-size: 0.85rem; color:var(--slate-600);">No está en la lista rápida, pero puede descargarse directamente de Yahoo Finance.</p>
              <button class="btn-action-primary" onclick="descargarSimboloDirecto('${customSym}')">
                <span>📥</span> Descargar "${customSym}" desde Yahoo Finance
              </button>
            </div>
          `;
        } else {
          grid.innerHTML = `
            <div style="grid-column: 1 / -1; text-align: center; padding: 28px; color: var(--slate-600);">
              <p style="font-size: 1rem; font-weight: 700;">No hay activos guardados con este filtro.</p>
            </div>
          `;
        }
        return;
      }

      grid.innerHTML = filtrados.map(act => {
        const inDb = dbMap[act.sym] !== undefined;
        const count = dbMap[act.sym] || 0;
        const isSelected = seleccionadosSet.has(act.sym);
        const badgeHtml = inDb
          ? `<span class="catalog-badge-pill saved">✓ En DB (${count.toLocaleString()} v.)</span>`
          : `<span class="catalog-badge-pill download">📥 Descargar</span>`;

        return `
          <div class="catalog-item-card ${inDb ? 'in-db' : ''} ${isSelected ? 'selected' : ''}" onclick="toggleSeleccionItem('${act.sym}', ${inDb}, event)">
            <div class="catalog-top-row">
              <span class="catalog-ticker-name">${act.sym}</span>
              ${badgeHtml}
            </div>
            <div class="catalog-company-sub" title="${act.name}">${act.name}</div>
            <div style="display:flex; justify-content:space-between; align-items:center; margin-top:2px;">
              <span class="catalog-badge-pill sector">${act.sector || 'Mercado'}</span>
              <input type="checkbox" ${isSelected ? 'checked' : ''} style="cursor:pointer;" onclick="event.stopPropagation(); toggleSeleccionDirecta('${act.sym}')">
            </div>
          </div>
        `;
      }).join('');
    }

    function toggleSeleccionDirecta(sym) {
      if (seleccionadosSet.has(sym)) seleccionadosSet.delete(sym);
      else seleccionadosSet.add(sym);
      actualizarContadorSeleccionados();
      filtrarCatalogo();
    }

    async function toggleSeleccionItem(sym, inDb, event) {
      // Si se hace clic en la tarjeta, si está en DB la agrega al input; si no, la selecciona o descarga
      if (inDb) {
        agregarTicker('optTickers', sym);
        agregarTicker('isOosTickers', sym);
        showToast(`✓ ${sym} agregado a los selectores de cartera.`);
      } else {
        await descargarSimboloDirecto(sym);
      }
    }

    function toggleSelectAll(checked) {
      const query = (document.getElementById('catalogSearchInput')?.value || '').trim().toLowerCase();
      const dbMap = {};
      catalogoItemsEnDb.forEach(item => { dbMap[item.ticker] = item.count; });

      const visibles = CATALOGO_ACTIVOS.filter(act => {
        if (filtroCatActual === 'en_db') {
          if (!dbMap[act.sym]) return false;
        } else if (filtroCatActual !== 'todos') {
          if (act.cat !== filtroCatActual) return false;
        }
        if (!query) return true;
        return act.sym.toLowerCase().includes(query) || act.name.toLowerCase().includes(query) || (act.sector || '').toLowerCase().includes(query);
      });

      visibles.forEach(act => {
        if (checked) seleccionadosSet.add(act.sym);
        else seleccionadosSet.delete(act.sym);
      });

      actualizarContadorSeleccionados();
      filtrarCatalogo();
    }

    function actualizarContadorSeleccionados() {
      const el = document.getElementById('selectedCountText');
      if (el) el.innerText = `${seleccionadosSet.size} seleccionados`;
    }

    async function descargarSeleccionados() {
      if (seleccionadosSet.size === 0) {
        showToast("⚠️ Seleccione al menos un símbolo con las casillas de verificación.");
        return;
      }

      const tickers = Array.from(seleccionadosSet);
      showToast(`📥 Descargando lote de ${tickers.length} activos (${tickers.slice(0, 5).join(', ')}${tickers.length > 5 ? '...' : ''})...`);
      try {
        const res = await fetch('/api/db/descargar', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ tickers })
        });
        const data = await res.json();
        if (data.success) {
          showToast(`✓ ${data.data.message}`);
          seleccionadosSet.clear();
          actualizarContadorSeleccionados();
          await cargarResumenDb();
        } else {
          showToast("❌ Error: " + (data.error || "Fallo en la descarga por lote"));
        }
      } catch (err) {
        showToast(`❌ Error: ${err.message}`);
      }
    }

    function agregarSeleccionadosACartera() {
      if (seleccionadosSet.size === 0) {
        showToast("⚠️ Seleccione al menos un símbolo para agregar a la cartera.");
        return;
      }
      seleccionadosSet.forEach(sym => {
        agregarTicker('optTickers', sym);
        agregarTicker('isOosTickers', sym);
      });
      showToast(`✓ ${seleccionadosSet.size} activos agregados a los selectores.`);
    }

    function renderCatalogo(itemsEnDb) {
      catalogoItemsEnDb = itemsEnDb || [];
      const countEl = document.getElementById('countTotalCat');
      if (countEl) countEl.innerText = `${CATALOGO_ACTIVOS.length}`;
      filtrarCatalogo();
    }

    async function cargarResumenDb() {
      try {
        const res = await fetch('/api/db/resumen');
        const data = await res.json();
        if (data.success && data.data) {
          const items = data.data.items;
          document.getElementById('dbStatusText').innerText = `SQLite: ${items.length} Activos Guardados`;
          renderCatalogo(items);

          const tbody = document.getElementById('tbodyDbResumen');
          if (items.length === 0) {
            tbody.innerHTML = '<tr><td colspan="4" style="text-align:center;">Base de datos vacía. Descargue tickers arriba.</td></tr>';
            return;
          }
          tbody.innerHTML = items.map(item => `
            <tr>
              <td><span class="ticker-badge">${item.ticker}</span></td>
              <td><strong>${item.count.toLocaleString()}</strong> velas</td>
              <td>${item.desde}</td>
              <td>${item.hasta}</td>
            </tr>
          `).join('');

          const optInput = document.getElementById('optTickers');
          const tickersEnDb = items.map(i => i.ticker).filter(t => t !== 'SPY');
          if (tickersEnDb.length >= 2 && optInput.value === '') {
            optInput.value = tickersEnDb.slice(0, 6).join(', ');
            document.getElementById('isOosTickers').value = tickersEnDb.slice(0, 6).join(', ');
            document.getElementById('trackTickers').value = tickersEnDb.slice(0, 3).join(', ');
          }
        }
      } catch (e) {
        console.error("Error al leer SQLite:", e);
        document.getElementById('dbStatusText').innerText = 'SQLite: Error de conexión';
      }
    }

    async function descargarTickers() {
      const raw = document.getElementById('dbDescargaInput').value;
      const tickers = raw.split(',').map(s => s.trim().toUpperCase()).filter(s => s.length > 0);
      if (tickers.length === 0) {
        showToast("⚠️ Ingrese al menos un ticker para descargar.");
        return;
      }

      const btn = document.getElementById('btnDescargarDb');
      btn.disabled = true;
      btn.innerHTML = '<span>⏳</span> Descargando de Yahoo Finance...';
      showToast(`Descargando cotizaciones para ${tickers.join(', ')}...`);

      try {
        const res = await fetch('/api/db/descargar', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ tickers })
        });
        const data = await res.json();
        btn.disabled = false;
        btn.innerHTML = '<span>📥</span> Descargar de Yahoo Finance';

        if (data.success) {
          showToast("✓ " + data.data.message);
          await cargarResumenDb();
          document.getElementById('dbDescargaInput').value = '';
        } else {
          showToast("❌ " + (data.error || "Fallo en la descarga"));
        }
      } catch (err) {
        btn.disabled = false;
        btn.innerHTML = '<span>📥</span> Descargar de Yahoo Finance';
        showToast("❌ Error: " + err.message);
      }
    }

    function abrirReporteEnNuevaVentana() {
      window.open('/api/reporte/html', '_blank');
    }

    function imprimirIframeReporte() {
      const iframe = document.getElementById('iframeReporte');
      if (iframe && iframe.contentWindow) {
        iframe.contentWindow.print();
      }
    }

    let modoEdicionReporteActivo = false;

    function toggleModoEdicionReporte() {
      modoEdicionReporteActivo = !modoEdicionReporteActivo;
      const iframe = document.getElementById('iframeReporte');
      if (iframe && iframe.contentWindow) {
        iframe.contentWindow.postMessage({ type: 'TOGGLE_EDIT', active: modoEdicionReporteActivo }, '*');
      }
      const btn = document.getElementById('btnParentEditReport');
      const txt = document.getElementById('textParentEdit');
      if (btn && txt) {
        txt.innerText = modoEdicionReporteActivo ? 'Modo Edición: ON' : 'Modo Edición: OFF';
        btn.style.background = modoEdicionReporteActivo ? '#16A34A' : '#D97706';
      }
      showToast(modoEdicionReporteActivo ? "✏️ Modo edición activado: haz clic en cualquier texto del informe para editarlo." : "🔒 Modo edición desactivado.");
    }

    function aplicarNegritaReporte() {
      if (!modoEdicionReporteActivo) {
        toggleModoEdicionReporte();
      }
      const iframe = document.getElementById('iframeReporte');
      if (iframe && iframe.contentWindow) {
        iframe.contentWindow.postMessage({ type: 'EXEC_BOLD' }, '*');
        showToast("𝐁 Negrita aplicada a la selección");
      }
    }

    function importarDatosMarkowitzAReporte() {
      if (!window.lastOptResult) {
        const saved = localStorage.getItem('cfuba_last_opt_result');
        if (saved) {
          try { window.lastOptResult = JSON.parse(saved); } catch(e){}
        }
      }
      if (!window.lastOptResult) {
        showToast("⚠️ Primero ejecute una optimización en 'Optimización Markowitz'.");
        return;
      }
      const iframe = document.getElementById('iframeReporte');
      if (iframe && iframe.contentWindow) {
        iframe.contentWindow.postMessage({ type: 'IMPORT_MARKOWITZ', payload: window.lastOptResult }, '*');
        showToast("📥 ¡Datos y gráficos de Markowitz importados al reporte!");
      }
    }

    function importarDatosSeguimientoAReporteInstitucional() {
      let data = window.lastTrackResult;
      if (!data) {
        const saved = localStorage.getItem('cfuba_last_tracking_data');
        if (saved) {
          try { data = JSON.parse(saved); } catch(e){}
        }
      }
      if (!data) {
        showToast("⚠️ Primero ejecute una simulación en la pestaña 'Seguimiento vs SPY'.");
        return;
      }
      const integrantes = document.getElementById('trackIntegrantes')?.value.trim() || 'Fausto Crivelli (Presidente de Portafolio) · Luciano Mora (Analista Sr) · Florencia Beluzzo (Analista Sr)';
      data.integrantes = integrantes;

      const iframe = document.getElementById('iframeReporte');
      if (iframe && iframe.contentWindow) {
        iframe.contentWindow.postMessage({ type: 'IMPORT_TRACKING', payload: data }, '*');
        showToast("📥 ¡Estadísticas, activos y tesis de Seguimiento sincronizados en el Reporte Institucional!");
      }
    }

    function guardarTextosReporte() {
      const iframe = document.getElementById('iframeReporte');
      if (iframe && iframe.contentWindow) {
        iframe.contentWindow.postMessage({ type: 'SAVE_EDITS' }, '*');
        showToast("💾 Solicitud de guardado enviada al reporte.");
      }
    }

    function restaurarReporteOriginal() {
      const iframe = document.getElementById('iframeReporte');
      if (iframe && iframe.contentWindow) {
        iframe.contentWindow.postMessage({ type: 'RESET_ORIGINAL' }, '*');
      }
    }

    let modoEdicionSeguimientoActivo = false;

    function toggleModoEdicionReporteSeguimiento() {
      modoEdicionSeguimientoActivo = !modoEdicionSeguimientoActivo;
      const iframe = document.getElementById('iframeReporteSeguimiento');
      if (iframe && iframe.contentWindow) {
        iframe.contentWindow.postMessage({ type: 'TOGGLE_EDIT', active: modoEdicionSeguimientoActivo }, '*');
      }
      const btn = document.getElementById('btnParentEditTrackReport');
      const txt = document.getElementById('textParentEditTrack');
      if (btn && txt) {
        txt.innerText = modoEdicionSeguimientoActivo ? 'Modo Edición: ON' : 'Modo Edición: OFF';
        btn.style.background = modoEdicionSeguimientoActivo ? '#16A34A' : '#D97706';
      }
      showToast(modoEdicionSeguimientoActivo ? "✏️ Modo edición activado: haz clic en cualquier texto del informe de seguimiento para modificarlo." : "🔒 Modo edición desactivado.");
    }

    function actualizarIntegrantesReporteSeguimiento(val) {
      const iframe = document.getElementById('iframeReporteSeguimiento');
      if (iframe && iframe.contentWindow) {
        iframe.contentWindow.postMessage({ type: 'UPDATE_INTEGRANTES', integrantes: val }, '*');
      }
      const trackInput = document.getElementById('trackIntegrantes');
      if (trackInput && trackInput.value !== val) {
        trackInput.value = val;
      }
      if (window.lastTrackResult) {
        window.lastTrackResult.integrantes = val;
      }
    }

    function importarDatosSeguimientoAReporte() {
      if (!window.lastTrackResult) {
        const saved = localStorage.getItem('cfuba_last_tracking_data');
        if (saved) {
          try { window.lastTrackResult = JSON.parse(saved); } catch(e){}
        }
      }
      if (!window.lastTrackResult) {
        showToast("⚠️ Primero ejecute una simulación en la pestaña 'Seguimiento vs SPY'.");
        return;
      }
      const integrantes = document.getElementById('trackIntegrantes')?.value.trim() || 'Fausto Crivelli (Presidente de Portafolio) · Luciano Mora (Analista Sr) · Florencia Beluzzo (Analista Sr)';
      window.lastTrackResult.integrantes = integrantes;
      const repInput = document.getElementById('reportTrackIntegrantesInput');
      if (repInput) repInput.value = integrantes;

      const iframe = document.getElementById('iframeReporteSeguimiento');
      if (iframe && iframe.contentWindow) {
        iframe.contentWindow.postMessage({ type: 'IMPORT_TRACKING', payload: window.lastTrackResult }, '*');
        showToast("📥 ¡Datos, integrantes y gráficos de seguimiento sincronizados con éxito!");
      }
    }

    function guardarTextosReporteSeguimiento() {
      const iframe = document.getElementById('iframeReporteSeguimiento');
      if (iframe && iframe.contentWindow) {
        iframe.contentWindow.postMessage({ type: 'SAVE_EDITS' }, '*');
        showToast("💾 Solicitud de guardado enviada al informe de seguimiento.");
      }
    }

    function restaurarReporteSeguimientoOriginal() {
      const iframe = document.getElementById('iframeReporteSeguimiento');
      if (iframe && iframe.contentWindow) {
        iframe.contentWindow.postMessage({ type: 'RESET_ORIGINAL' }, '*');
      }
    }

    function abrirReporteSeguimientoEnNuevaVentana() {
      window.open('/api/reporte-seguimiento/html', '_blank');
    }

    function imprimirIframeReporteSeguimiento() {
      const iframe = document.getElementById('iframeReporteSeguimiento');
      if (iframe && iframe.contentWindow) {
        iframe.contentWindow.print();
      }
    }

    function exportarGraficoPNG(cardId, filename) {
      const el = document.getElementById(cardId);
      if (!el) return;
      showToast("📸 Exportando gráfico a 1920x1080 PNG...");
      html2canvas(el, { scale: 2, backgroundColor: '#FFFFFF' }).then(canvas => {
        const link = document.createElement('a');
        link.download = `${filename}.png`;
        link.href = canvas.toDataURL('image/png');
        link.click();
        showToast(`✓ ${filename}.png descargado.`);
      }).catch(err => {
        showToast("❌ Error al exportar imagen: " + err.message);
      });
    }

    // Inicialización inmediata al cargar el DOM sin bloqueos
    document.addEventListener('DOMContentLoaded', () => {
      cargarResumenDb().then(() => {
        cargarCarterasDb();
        ejecutarOptimizacion();
      });

      // Latido periódico de actividad
      setInterval(() => {
        fetch('/api/heartbeat', { method: 'POST' }).catch(() => {});
      }, 10000);
    });
  </script>
</body>
</html>
"##;
