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

    @keyframes spin {
      to { transform: rotate(360deg); }
    }
  </style>
</head>
<body>

  <!-- Barra de Navegación Superior -->
  <header class="top-navbar">
    <div class="brand-section">
      <div class="brand-logo-badge">UBA</div>
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
      <button class="menu-item-btn" onclick="switchView('view-db')">
        <span class="menu-icon">🗄️</span> Base de Datos SQLite
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
              <button class="btn-export-1080p" onclick="exportarGraficoPNG('cardIsOosEvolucion', 'Is_Oos_Curva_Equidad')">📸 PNG 1080p</button>
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

        <div class="card-panel">
          <div class="input-group">
            <label>Activos de la Cartera</label>
            <input type="text" id="trackTickers" class="input-control" value="AAPL, YPF, AMZN">
          </div>
          <div class="form-grid-controls">
            <div class="input-group">
              <label>Ponderaciones % (ej: 40, 30, 30)</label>
              <input type="text" id="trackPesos" class="input-control" value="40, 30, 30" placeholder="Ponderaciones separadas por coma">
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

        <div class="charts-grid-2x2">
          <div class="chart-panel-card" id="cardTrackEquity" style="grid-column: span 2;">
            <div class="chart-panel-header">
              <span class="chart-panel-title">Curva de Equidad de la Cartera vs SPY (Base 1.0)</span>
              <button class="btn-export-1080p" onclick="exportarGraficoPNG('cardTrackEquity', 'Curva_Equidad_Seguimiento')">📸 PNG 1080p</button>
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
            <p>Maquetación oficial de 6 páginas con fichas de tesis de inversión y gráficos en alta definición.</p>
          </div>
          <div style="display: flex; gap: 10px;">
            <button class="btn-action-primary" onclick="abrirReporteEnNuevaVentana()">
              <span>↗️</span> Abrir en Pestaña Completa
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

        <div class="card-panel">
          <div class="input-group">
            <label>Descargar / Actualizar Símbolo Personalizado (Separados por coma)</label>
            <input type="text" id="dbDescargaInput" class="input-control" placeholder="Ej: GOOGL, NVDA, TSLA, GEV, BMA">
          </div>
          <div style="display: flex; gap: 12px;">
            <button id="btnDescargarDb" class="btn-action-primary" onclick="descargarTickers()">
              <span>📥</span> Descargar de Yahoo Finance
            </button>
            <button class="btn-action-primary" style="background: var(--navy-900);" onclick="cargarResumenDb()">
              <span>🔄</span> Refrescar Tabla SQLite
            </button>
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
      </section>

    </main>
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

      if (viewId === 'view-db') cargarResumenDb();
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

      // 4. Evolución Acumulada
      if (charts.evolucion) charts.evolucion.destroy();
      const ctxEvolucion = document.getElementById('chartEvolucion').getContext('2d');
      const timeLabels = data.time_labels.slice(-60);
      const datasetsEvol = [];
      const colores = ['#0062FF', '#0A192F', '#00D2D3', '#F59E0B', '#10B981', '#EC4899', '#8B5CF6'];
      
      data.tickers.forEach((t, i) => {
        const series = data.series_map[t];
        if (series && series.length > 0) {
          const sliceP = series.slice(-60);
          const base = sliceP[0] || 100;
          datasetsEvol.push({
            label: t,
            data: sliceP.map(p => ((p / base) * 100).toFixed(1)),
            borderColor: colores[i % colores.length],
            borderWidth: 1.8,
            fill: false,
            tension: 0.1,
            pointRadius: 0
          });
        }
      });

      charts.evolucion = new Chart(ctxEvolucion, {
        type: 'line',
        data: { labels: timeLabels, datasets: datasetsEvol },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: { legend: { position: 'top' } },
          scales: { x: { display: false } }
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

      // 6. Matriz de Correlación
      const thead = document.getElementById('theadCorr');
      thead.innerHTML = '<th></th>' + data.tickers.map(t => `<th>${t}</th>`).join('');
      const tbody = document.getElementById('tbodyCorr');
      tbody.innerHTML = '';
      data.matriz_correlacion.forEach((fila, r) => {
        let rowHtml = `<td><strong>${data.tickers[r]}</strong></td>`;
        fila.forEach((val, c) => {
          const bg = r === c ? '#EBF3FA' : (val > 0.6 ? '#FEE2E2' : '#F8FAFC');
          rowHtml += `<td style="background:${bg}; font-family:'JetBrains Mono'; font-weight:700;">${val.toFixed(2)}</td>`;
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

        // 1. Gráfico Curva de Equidad Continua (IS vs OOS vs Benchmark)
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
        document.getElementById('kpiTrackGain').innerText = d.tracking_gain_pct.toFixed(1) + '%';
        document.getElementById('kpiTrackMaxDd').innerText = d.tracking_max_dd_pct.toFixed(1) + '%';
        document.getElementById('kpiTrackSharpe').innerText = d.tracking_sharpe.toFixed(2);
        document.getElementById('kpiTrackMaxStag').innerText = d.tracking_max_stagnation_days + ' días';

        if (charts.trackEquity) charts.trackEquity.destroy();
        const ctxEq = document.getElementById('chartTrackEquity').getContext('2d');
        charts.trackEquity = new Chart(ctxEq, {
          type: 'line',
          data: {
            labels: d.time_labels,
            datasets: [
              {
                label: 'Portafolio Personalizado (Equidad)',
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

      // 💻 Big Tech & Inteligencia Artificial
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
        grid.innerHTML = `
          <div style="grid-column: 1 / -1; text-align: center; padding: 28px; color: var(--slate-600);">
            <p style="font-size: 1.1rem; font-weight: 700; margin-bottom: 6px;">No se encontraron activos para "${query}".</p>
            <p style="font-size: 0.85rem;">Puede descargar cualquier símbolo personalizado escribiéndolo en el campo inferior.</p>
          </div>
        `;
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
        ejecutarOptimizacion();
      });

      // Enviar latido de actividad (heartbeat) cada 2 segundos a Rust
      setInterval(() => {
        fetch('/api/heartbeat', { method: 'POST' }).catch(() => {});
      }, 2000);

      // Notificar cierre inmediato al backend cuando se cierre la ventana o pestaña
      window.addEventListener('beforeunload', () => {
        if (navigator.sendBeacon) {
          navigator.sendBeacon('/api/shutdown');
        } else {
          fetch('/api/shutdown', { method: 'POST', keepalive: true }).catch(() => {});
        }
      });
    });
  </script>
</body>
</html>
"##;
