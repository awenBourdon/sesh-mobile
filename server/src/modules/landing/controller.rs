use crate::AppState;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use std::sync::Arc;

pub async fn landing_page(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let spot_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM spots")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    let trick_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM tricks WHERE is_approved = true")
            .fetch_one(&state.pool)
            .await
            .unwrap_or(0);

    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    Html(format!(r##"
        <!DOCTYPE html>
        <html lang="fr">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>SESH - L'Application N°1 des Skateurs | Trouve, Plaque, Partage</title>
            <meta name="description" content="Découvre la carte de skate communautaire. Spots en temps réel, vidéos brutes et alertes météo/sécurité. Télécharge SESH maintenant.">
            <style>
                * {{
                    box-sizing: border-box;
                    margin: 0;
                    padding: 0;
                }}
                body {{
                    font-family: -apple-system, BlinkMacSystemFont, 'Inter', 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
                    background-color: #F7F7F7;
                    color: #121212;
                    line-height: 1.5;
                    -webkit-font-smoothing: antialiased;
                    overflow-x: hidden;
                }}

                /* BADGES & ACCENTS */
                .badge_dark {{
                    background-color: #121212;
                    color: #FFFFFF !important;
                    font-weight: 800;
                    font-size: 0.75rem;
                    padding: 0.4rem 0.9rem;
                    border-radius: 20px;
                    text-transform: uppercase;
                    letter-spacing: 0.5px;
                    display: inline-block;
                }}
                .badge_outline {{
                    background-color: transparent;
                    color: #121212;
                    border: 1px solid #121212;
                    font-weight: 800;
                    font-size: 0.75rem;
                    padding: 0.4rem 0.9rem;
                    border-radius: 20px;
                    text-transform: uppercase;
                    letter-spacing: 0.5px;
                    display: inline-block;
                }}

                /* NAVBAR */
                .navbar {{
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 1.5rem 2rem;
                    max-width: 1200px;
                    margin: 0 auto;
                    position: sticky;
                    top: 0;
                    background: rgba(247, 247, 247, 0.9);
                    backdrop-filter: blur(10px);
                    z-index: 100;
                }}
                .logo {{
                    font-size: 2rem;
                    font-weight: 900;
                    letter-spacing: -2px;
                }}
                .nav_links {{
                    display: flex;
                    gap: 2rem;
                    align-items: center;
                }}
                .nav_links a {{
                    text-decoration: none;
                    color: #121212;
                    font-weight: 700;
                    font-size: 0.9rem;
                    transition: opacity 0.2s;
                }}
                .nav_links a.badge_dark {{
                    color: #FFFFFF !important;
                }}
                .nav_links a:hover {{
                    opacity: 0.6;
                }}

                /* BUTTONS */
                .cta_button {{
                    background-color: #121212;
                    color: #FFFFFF;
                    padding: 1rem 2rem;
                    border-radius: 24px;
                    text-decoration: none;
                    font-weight: 900;
                    font-size: 1rem;
                    transition: transform 0.15s ease, background-color 0.2s ease, box-shadow 0.2s ease;
                    display: inline-flex;
                    align-items: center;
                    gap: 0.6rem;
                    border: none;
                    cursor: pointer;
                    box-shadow: 0 4px 15px rgba(0,0,0,0.1);
                }}
                .cta_button:hover {{
                    background-color: #000000;
                    transform: translateY(-2px);
                    box-shadow: 0 6px 20px rgba(0,0,0,0.18);
                }}

                /* HERO SECTION */
                .hero {{
                    text-align: center;
                    padding: 5rem 1.5rem 4rem 1.5rem;
                    max-width: 900px;
                    margin: 0 auto;
                }}
                .hero h1 {{
                    font-size: clamp(2.8rem, 7vw, 5.2rem);
                    font-weight: 900;
                    letter-spacing: -3px;
                    line-height: 1.02;
                    margin: 1.5rem 0;
                    text-transform: uppercase;
                }}
                .hero p {{
                    font-size: clamp(1.1rem, 2.5vw, 1.4rem);
                    color: #555555;
                    max-width: 700px;
                    margin: 0 auto 2.5rem auto;
                    font-weight: 500;
                }}
                .hero_cta_group {{
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    gap: 1rem;
                }}
                .micro_trust {{
                    font-size: 0.85rem;
                    color: #777777;
                    font-weight: 600;
                }}

                /* PAIN POINT / PROBLEM SECTION */
                .problem_section {{
                    background: #FFFFFF;
                    border-top: 1px solid rgba(0,0,0,0.08);
                    border-bottom: 1px solid rgba(0,0,0,0.08);
                    padding: 6rem 1.5rem;
                }}
                .section_inner {{
                    max-width: 1100px;
                    margin: 0 auto;
                }}
                .section_title {{
                    font-size: clamp(2rem, 4vw, 3rem);
                    font-weight: 900;
                    letter-spacing: -1.5px;
                    margin-bottom: 1rem;
                    text-transform: uppercase;
                }}
                .section_subtitle {{
                    font-size: 1.1rem;
                    color: #666666;
                    margin-bottom: 3.5rem;
                    max-width: 600px;
                }}
                .problem_grid {{
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
                    gap: 2rem;
                }}
                .problem_card {{
                    padding: 2rem;
                    border-radius: 20px;
                    background: #F9F9F9;
                    border: 1px solid rgba(0,0,0,0.05);
                }}
                .problem_num {{
                    font-size: 0.8rem;
                    font-weight: 900;
                    color: #888888;
                    margin-bottom: 0.8rem;
                }}
                .problem_card h4 {{
                    font-size: 1.2rem;
                    font-weight: 900;
                    margin-bottom: 0.5rem;
                }}
                .problem_card p {{
                    color: #666666;
                    font-size: 0.95rem;
                }}

                /* STATS COUNTER */
                .stats_section {{
                    padding: 6rem 1.5rem;
                    background: #F7F7F7;
                }}
                .stats_grid {{
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
                    gap: 2rem;
                }}
                .stat_card {{
                    background: #FFFFFF;
                    border: 1px solid rgba(0,0,0,0.08);
                    border-radius: 28px;
                    padding: 3rem 2rem;
                    text-align: center;
                    box-shadow: 0 10px 30px rgba(0,0,0,0.03);
                }}
                .stat_number {{
                    font-size: 4rem;
                    font-weight: 900;
                    letter-spacing: -3px;
                    line-height: 1;
                    margin-bottom: 0.5rem;
                    color: #121212;
                }}
                .stat_label {{
                    font-size: 0.85rem;
                    font-weight: 900;
                    color: #888888;
                    letter-spacing: 1px;
                    text-transform: uppercase;
                }}

                /* FEATURES SHOWCASE */
                .features_section {{
                    background: #FFFFFF;
                    padding: 6rem 1.5rem;
                }}
                .feature_row {{
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
                    gap: 4rem;
                    align-items: center;
                    margin-bottom: 5rem;
                }}
                .feature_text h3 {{
                    font-size: 2rem;
                    font-weight: 900;
                    letter-spacing: -1px;
                    margin: 1rem 0;
                }}
                .feature_text p {{
                    color: #555555;
                    font-size: 1.05rem;
                    margin-bottom: 1.5rem;
                }}
                .feature_mockup {{
                    background: #121212;
                    border-radius: 32px;
                    padding: 2.5rem;
                    color: #FFFFFF;
                    min-height: 280px;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    box-shadow: 0 20px 40px rgba(0,0,0,0.15);
                }}
                .mockup_header {{
                    font-size: 0.75rem;
                    font-weight: 900;
                    color: #FFFFFF;
                    opacity: 0.6;
                    margin-bottom: 1rem;
                    letter-spacing: 1px;
                }}
                .mockup_body {{
                    font-size: 1.5rem;
                    font-weight: 900;
                    letter-spacing: -0.5px;
                    line-height: 1.3;
                }}

                /* TESTIMONIALS / PROOF */
                .testimonials_section {{
                    padding: 6rem 1.5rem;
                    background: #F7F7F7;
                }}
                .testimonial_grid {{
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                    gap: 2rem;
                }}
                .testimonial_card {{
                    background: #FFFFFF;
                    padding: 2.5rem;
                    border-radius: 24px;
                    border: 1px solid rgba(0,0,0,0.08);
                }}
                .testimonial_quote {{
                    font-size: 1.1rem;
                    font-weight: 700;
                    margin-bottom: 1.5rem;
                    line-height: 1.4;
                }}
                .testimonial_author {{
                    font-size: 0.85rem;
                    font-weight: 900;
                    color: #888888;
                }}

                /* FAQ SECTION */
                .faq_section {{
                    background: #FFFFFF;
                    padding: 6rem 1.5rem;
                    border-top: 1px solid rgba(0,0,0,0.08);
                }}
                .faq_list {{
                    max-width: 800px;
                    margin: 0 auto;
                }}
                .faq_item {{
                    border-bottom: 1px solid rgba(0,0,0,0.08);
                    padding: 2rem 0;
                }}
                .faq_question {{
                    font-size: 1.3rem;
                    font-weight: 900;
                    margin-bottom: 0.8rem;
                    letter-spacing: -0.5px;
                }}
                .faq_answer {{
                    color: #666666;
                    font-size: 1rem;
                    line-height: 1.6;
                }}

                /* FINAL CTA BANNER */
                .final_cta {{
                    background: #121212;
                    color: #FFFFFF;
                    text-align: center;
                    padding: 6rem 1.5rem;
                }}
                .final_cta h2 {{
                    font-size: clamp(2.5rem, 5vw, 4rem);
                    font-weight: 900;
                    letter-spacing: -2px;
                    margin-bottom: 1.5rem;
                    text-transform: uppercase;
                }}
                .final_cta p {{
                    font-size: 1.2rem;
                    color: #AAAAAA;
                    margin-bottom: 2.5rem;
                    max-width: 600px;
                    margin-left: auto;
                    margin-right: auto;
                }}

                /* FOOTER */
                .footer {{
                    text-align: center;
                    padding: 3rem 1.5rem;
                    background: #000000;
                    color: #666666;
                    font-size: 0.85rem;
                    font-weight: 600;
                }}
            </style>
        </head>
        <body>

            <!-- NAVBAR -->
            <nav class="navbar">
                <div class="logo">SESH</div>
                <div class="nav_links">
                    <a href="#features">Fonctionnalités</a>
                    <a href="#stats">Statistiques</a>
                    <a href="#faq">FAQ</a>
                    <a href="#download" class="badge_dark" style="text-decoration: none; padding: 0.6rem 1.2rem; color: #FFFFFF;">RIDER MAINTENANT</a>
                </div>
            </nav>

            <!-- HERO SECTION -->
            <header class="hero">
                <span class="badge_outline">L'App N°1 de la Culture Skate</span>
                <h1>Ne gâche plus jamais une session.</h1>
                <p>Trouve les vrais spots autour de toi, vérifie l'état du sol en temps réel, découvre les vidéos brutes des locals et partage tes meilleurs tricks.</p>
                <div class="hero_cta_group">
                    <a href="#download" class="cta_button">
                        <span>TÉLÉCHARGER POUR ANDROID</span>
                    </a>
                    <span class="micro_trust">100% Gratuit &bull; Fait par des skateurs &bull; Aucun compte requis pour explorer</span>
                </div>
            </header>

            <!-- PAIN POINTS / PROBLEM -->
            <section class="problem_section">
                <div class="section_inner">
                    <span class="badge_dark">Le Problème</span>
                    <h2 class="section_title" style="margin-top: 0.8rem;">Marre des sessions foirées ?</h2>
                    <p class="section_subtitle">On a tous déjà fait 45 minutes de trajet pour arriver sur un spot en travaux, sous la pluie ou blindé de sécurité.</p>

                    <div class="problem_grid">
                        <div class="problem_card">
                            <div class="problem_num">01 / INFOS PÉRIMÉES</div>
                            <h4>Cartes pas à jour</h4>
                            <p>Les cartes classiques ne te disent jamais si le sol est mouillé ou si le curb a été détruit hier.</p>
                        </div>
                        <div class="problem_card">
                            <div class="problem_num">02 / SÉCURITÉ & TRAVAUX</div>
                            <h4>Déplacements inutiles</h4>
                            <p>Arriver sur un spot mythique et se faire virer en 30 secondes parce qu'une alarme a été posée.</p>
                        </div>
                        <div class="problem_card">
                            <div class="problem_num">03 / VIDÉOS FILTRÉES</div>
                            <h4>Réseaux pollués</h4>
                            <p>Les réseaux sociaux affichent des algorithmes compliqués au lieu des vrais clips bruts du spot.</p>
                        </div>
                    </div>
                </div>
            </section>

            <!-- LIVE STATS -->
            <section class="stats_section" id="stats">
                <div class="section_inner">
                    <div style="text-align: center;">
                        <span class="badge_outline">Données Réelles NeonDB</span>
                        <h2 class="section_title" style="margin-top: 0.8rem;">SESH en Chiffres</h2>
                    </div>
                    <div class="stats_grid" style="margin-top: 3rem;">
                        <div class="stat_card">
                            <div class="stat_number">{}</div>
                            <div class="stat_label">SPOTS VÉRIFIÉS</div>
                        </div>
                        <div class="stat_card">
                            <div class="stat_number">{}</div>
                            <div class="stat_label">TRICKS PLAQUÉS</div>
                        </div>
                        <div class="stat_card">
                            <div class="stat_number">{}</div>
                            <div class="stat_label">SKATEURS ACTIFS</div>
                        </div>
                    </div>
                </div>
            </section>

            <!-- FEATURES SHOWCASE -->
            <section class="features_section" id="features">
                <div class="section_inner">

                    <div class="feature_row">
                        <div class="feature_text">
                            <span class="badge_dark">01 / CARTE SATELLITE</span>
                            <h3>Trouve le béton parfait près de chez toi.</h3>
                            <p>Une vue satellite haute résolution pour repérer les curbs, les marches et les bikeparks. Filtre par distance et découvre des spots cachés.</p>
                        </div>
                        <div class="feature_mockup">
                            <div class="mockup_header">CARTE INTRACTIVE</div>
                            <div class="mockup_body">"Curb en marbre à 300 mètres. Prêt pour la session."</div>
                        </div>
                    </div>

                    <div class="feature_row">
                        <div class="feature_mockup" style="background: #1A1A1A;">
                            <div class="mockup_header">CLOUDINARY POWERED</div>
                            <div class="mockup_body">"Aperçu vidéo instantané sans attente de chargement."</div>
                        </div>
                        <div class="feature_text">
                            <span class="badge_dark">02 / FEED BRUT</span>
                            <h3>Des clips réels, pas du contenu retouché.</h3>
                            <p>Chaque spot possède son propre feed vidéo. Consulte les tricks plaqués par les locaux et poste les tiens en un clic.</p>
                        </div>
                    </div>

                    <div class="feature_row">
                        <div class="feature_text">
                            <span class="badge_dark">03 / ALERTES COMMUNAUTAIRES</span>
                            <h3>Sache avant de te déplacer.</h3>
                            <p>Laisse tes commentaires sur l'état du sol, la présence de sécurité ou la météo. La communauté te tient informé en direct.</p>
                        </div>
                        <div class="feature_mockup">
                            <div class="mockup_header">ÉTAT DU SPOT</div>
                            <div class="mockup_body">"Sol sec, sécurité partie. Foncez !"</div>
                        </div>
                    </div>

                </div>
            </section>

            <!-- TESTIMONIALS / PROOF -->
            <section class="testimonials_section">
                <div class="section_inner">
                    <div style="text-align: center; margin-bottom: 4rem;">
                        <span class="badge_dark">La Communauté</span>
                        <h2 class="section_title" style="margin-top: 0.8rem;">Ce que disent les skateurs</h2>
                    </div>

                    <div class="testimonial_grid">
                        <div class="testimonial_card">
                            <div class="testimonial_quote">"J'ai découvert 3 spots en marbre à 10min de chez moi que je ne connaissais même pas après 5 ans de skate."</div>
                            <div class="testimonial_author">&mdash; LEO M. (PARIS)</div>
                        </div>
                        <div class="testimonial_card">
                            <div class="testimonial_quote">"Le fait de voir si le sol est sec avant de prendre le métro m'a sauvé au moins 10 sessions cet hiver."</div>
                            <div class="testimonial_author">&mdash; ALEX T. (LYON)</div>
                        </div>
                        <div class="testimonial_card">
                            <div class="testimonial_quote">"Le feed vidéo sans fioritures est trop lourd. Tu vois direct quel niveau il faut pour le spot."</div>
                            <div class="testimonial_author">&mdash; SAMIR K. (BORDEAUX)</div>
                        </div>
                    </div>
                </div>
            </section>

            <!-- FAQ SECTION -->
            <section class="faq_section" id="faq">
                <div class="section_inner">
                    <div style="text-align: center; margin-bottom: 4rem;">
                        <span class="badge_outline">Questions Fréquentes</span>
                        <h2 class="section_title" style="margin-top: 0.8rem;">Tout ce que tu dois savoir</h2>
                    </div>

                    <div class="faq_list">
                        <div class="faq_item">
                            <div class="faq_question">L'application SESH est-elle gratuite ?</div>
                            <div class="faq_answer">Oui, 100% gratuite. L'application est développée par des passionnés pour la communauté skate.</div>
                        </div>
                        <div class="faq_item">
                            <div class="faq_question">Comment sont ajoutés et modérés les spots ?</div>
                            <div class="faq_answer">N'importe quel utilisateur peut proposer un spot ou un trick. Notre système de modération rapide valide le contenu pour garder la carte propre.</div>
                        </div>
                        <div class="faq_item">
                            <div class="faq_question">Sur quelles plateformes l'application est-elle disponible ?</div>
                            <div class="faq_answer">Disponible immédiatement sur Android (fichier APK / Play Store) et bientôt sur iOS.</div>
                        </div>
                    </div>
                </div>
            </section>

            <!-- FINAL CTA -->
            <section class="final_cta" id="download">
                <span class="badge_outline" style="color: #FFFFFF; border-color: #FFFFFF;">REJOINS LA SESSION</span>
                <h2>Prêt à trouver ton prochain spot ?</h2>
                <p>Télécharge SESH maintenant et accède gratuitement à la carte communautaire.</p>
                <a href="#" class="cta_button" style="background: #FFFFFF; color: #121212; padding: 1.2rem 2.5rem; font-size: 1.1rem;">
                    <span>TÉLÉCHARGER L'APPLICATION</span>
                </a>
            </section>

            <!-- FOOTER -->
            <footer class="footer">
                &copy; 2026 SESH. Conçu pour la culture skate.
            </footer>

        </body>
        </html>
    "##, spot_count, trick_count, user_count)).into_response()
}
