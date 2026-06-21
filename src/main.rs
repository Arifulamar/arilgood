use axum::{response::Html, routing::get, Router};
use maud::{html, Markup, DOCTYPE};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(portfolio_page));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Portofolio Rust berjalan di http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn portfolio_page() -> Html<String> {
    let markup = html! {
        (DOCTYPE)
        html lang="en" class="scroll-smooth" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Ariful Amar | Digital Creative Consultant" }
                
                // Tailwind CSS v4 via String Literal (Aman dari error Maud)
                script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {}
                
                // Font Awesome Icons
                link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css";
            }
            body class="bg-slate-950 text-slate-100 font-sans antialiased selection:bg-cyan-500 selection:text-slate-950" {
                
                // --- NAVBAR ---
                nav class="sticky top-0 z-50 backdrop-blur-md bg-slate-950/80 border-b border-slate-800" {
                    div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between" {
                        div class="flex items-center gap-2" {
                            span class="text-xl font-black tracking-wider bg-gradient-to-r from-cyan-400 to-blue-500 text-transparent bg-clip-text" { "ARIFUL AMAR" }
                        }
                        div class="hidden md:flex items-center space-y-0 space-x-8 text-sm font-medium text-slate-300" {
                            a href="#home" class="hover:text-cyan-400 transition" { "Home" }
                            a href="#about" class="hover:text-cyan-400 transition" { "About" }
                            a href="#services" class="hover:text-cyan-400 transition" { "Services" }
                            a href="#experience" class="hover:text-cyan-400 transition" { "Experience" }
                            a href="https://www.instagram.com/ariful.amar/" target="_blank" class="hover:text-cyan-400 transition text-lg" { i class="fab fa-instagram" {} }
                        }
                    }
                }

                // --- HERO SECTION ---
                section id="home" class="relative min-h-[85vh] flex items-center justify-center overflow-hidden pt-16" {
                    div class="absolute inset-0 bg-[radial-gradient(circle_at_30%_30%,rgba(6,182,212,0.1),transparent_50%)]" {}
                    div class="max-w-4xl mx-auto px-4 text-center z-10" {
                        span class="inline-flex items-center gap-2 px-3 py-1 rounded-full text-xs font-semibold bg-cyan-500/10 text-cyan-400 border border-cyan-500/20 mb-6" {
                            span class="w-2 h-2 rounded-full bg-cyan-400 animate-pulse" {} "Let's Grow Up!"
                        }
                        h1 class="text-4xl sm:text-6xl font-extrabold tracking-tight text-white mb-6" {
                            "Digital Creative "
                            span class="bg-gradient-to-r from-cyan-400 to-blue-500 text-transparent bg-clip-text" { "Expert & Consultant" }
                        }
                        p class="text-lg sm:text-xl text-slate-400 max-w-2xl mx-auto mb-10 leading-relaxed" {
                            "We help you start from planning, strategy, execution, monitoring to evaluation of your Digital Brand. The goal is only one, make your business grow up!"
                        }
                        div class="flex flex-col sm:flex-row justify-center items-center gap-4" {
                            a href="https://wa.me/6281390030856" target="_blank" class="w-full sm:w-auto px-8 py-3.5 bg-gradient-to-r from-cyan-500 to-blue-600 hover:from-cyan-400 hover:to-blue-500 text-slate-950 font-bold rounded-xl shadow-lg shadow-cyan-500/20 transition transform hover:-translate-y-0.5 text-center" {
                                i class="fab fa-whatsapp mr-2" {} "Chat Me"
                            }
                            a href="#about" class="w-full sm:w-auto px-8 py-3.5 bg-slate-800 hover:bg-slate-700 text-white font-semibold rounded-xl border border-slate-700 transition text-center" {
                                "Read More"
                            }
                        }
                    }
                }

                // --- ABOUT ME ---
                section id="about" class="py-24 border-t border-slate-900 bg-slate-900/30" {
                    div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8" {
                        div class="grid grid-cols-1 md:grid-cols-12 gap-12 items-center" {
                            // Avatar Placeholder Box
                            div class="md:col-span-4 flex justify-center" {
                                div class="relative group" {
                                    div class="absolute -inset-1 rounded-2xl bg-gradient-to-r from-cyan-500 to-blue-500 opacity-30 group-hover:opacity-50 blur transition duration-500" {}
                                    div class="relative w-64 h-64 bg-slate-800 rounded-2xl border border-slate-700 flex flex-col items-center justify-center p-6 shadow-xl" {
                                        div class="w-24 h-24 rounded-full bg-slate-700 flex items-center justify-center mb-4" {
                                            i class="fas fa-user-tie text-4xl text-cyan-400" {}
                                        }
                                        h3 class="text-xl font-bold text-white text-center" { "Ariful Amar" }
                                        p class="text-xs text-cyan-400 mt-1 font-mono" { "CEO @ Adsea.id" }
                                    }
                                    div class="absolute -bottom-4 -right-4 bg-slate-800 border border-slate-700 rounded-lg p-3 shadow-lg" {
                                        p class="text-xs text-slate-400" { "Est. Since" }
                                        p class="text-sm font-bold text-emerald-400" { "2019" }
                                    }
                                }
                            }
                            // Text Bio
                            div class="md:col-span-8 space-y-6" {
                                h2 class="text-3xl font-bold text-white tracking-tight flex items-center gap-3" {
                                    span class="w-8 h-1 bg-cyan-400 rounded-full" {} "About Me"
                                }
                                p class="text-slate-300 leading-relaxed text-lg" {
                                    "Actually, I am a " span class="text-cyan-400 font-semibold" { "Digital Brand Builder" } ". I am very passionate when talking about innovation and providing solutions for others because my job as a Digital Brand Builder is more passionate to always give the best to my customers."
                                }
                                p class="text-slate-400 leading-relaxed" {
                                    "Serving as CEO at Adsea.id—the company I founded in 2019—has brought me to meet incredible people and provided me with complex, rich managerial experience across various tech and creative domains."
                                }
                            }
                        }
                    }
                }

                // --- SERVICES ---
                section id="services" class="py-24 border-t border-slate-900" {
                    div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8" {
                        div class="text-center max-w-2xl mx-auto mb-16" {
                            h2 class="text-3xl font-bold text-white mb-4" { "Our Professional Services" }
                            p class="text-slate-400" { "Tailored solutions to digitalize your business and capture wider audiences." }
                        }
                        
                        div class="grid grid-cols-1 md:grid-cols-3 gap-6" {
                            // UI/UX
                            div class="p-6 bg-slate-900/50 rounded-xl border border-slate-800 hover:border-cyan-500/50 transition duration-300" {
                                div class="w-12 h-12 rounded-lg bg-cyan-500/10 flex items-center justify-center text-cyan-400 text-xl mb-4" { i class="fas fa-compass" {} }
                                h3 class="text-xl font-bold text-white mb-2" { "UI/UX Design" }
                                p class="text-sm text-slate-400 leading-relaxed" { "We collaborate with UI/UX experts who provide the best industry-standard interfaces and seamless user journeys for your clients." }
                            }
                            // Web Developer
                            div class="p-6 bg-slate-900/50 rounded-xl border border-slate-800 hover:border-cyan-500/50 transition duration-300" {
                                div class="w-12 h-12 rounded-lg bg-blue-500/10 flex items-center justify-center text-blue-400 text-xl mb-4" { i class="fas fa-code" {} }
                                h3 class="text-xl font-bold text-white mb-2" { "Web Development" }
                                p class="text-sm text-slate-400 leading-relaxed" { "Custom web application development using high-performance frameworks such as Laravel, Express JS, CodeIgniter, or optimized systems." }
                            }
                            // Mobile Apps
                            div class="p-6 bg-slate-900/50 rounded-xl border border-slate-800 hover:border-cyan-500/50 transition duration-300" {
                                div class="w-12 h-12 rounded-lg bg-purple-500/10 flex items-center justify-center text-purple-400 text-xl mb-4" { i class="fas fa-mobile-alt" {} }
                                h3 class="text-xl font-bold text-white mb-2" { "Mobile Apps" }
                                p class="text-sm text-slate-400 leading-relaxed" { "Recently, Mobile Apps have become an effective marketing channel to keep businesses seamlessly connected with their core audience." }
                            }
                            // Software Engineering
                            div class="p-6 bg-slate-900/50 rounded-xl border border-slate-800 hover:border-cyan-500/50 transition duration-300" {
                                div class="w-12 h-12 rounded-lg bg-emerald-500/10 flex items-center justify-center text-emerald-400 text-xl mb-4" { i class="fas fa-laptop-code" {} }
                                h3 class="text-xl font-bold text-white mb-2" { "Software Engineering" }
                                p class="text-sm text-slate-400 leading-relaxed" { "Your business needs dedicated internal computing for business processes, both offline and online. We are ready to scale it up." }
                            }
                            // Photoshop & Premiere
                            div class="p-6 bg-slate-900/50 rounded-xl border border-slate-800 hover:border-cyan-500/50 transition duration-300" {
                                div class="w-12 h-12 rounded-lg bg-amber-500/10 flex items-center justify-center text-amber-400 text-xl mb-4" { i class="fas fa-film" {} }
                                h3 class="text-xl font-bold text-white mb-2" { "Photoshop & Premiere" }
                                p class="text-sm text-slate-400 leading-relaxed" { "We meticulously design advertising production materials on the Internet with Adobe Premiere for high-converting video and Photoshop for graphics." }
                            }
                            // Graphic Design
                            div class="p-6 bg-slate-900/50 rounded-xl border border-slate-800 hover:border-cyan-500/50 transition duration-300" {
                                div class="w-12 h-12 rounded-lg bg-rose-500/10 flex items-center justify-center text-rose-400 text-xl mb-4" { i class="fas fa-palette" {} }
                                h3 class="text-xl font-bold text-white mb-2" { "Graphic Design" }
                                p class="text-sm text-slate-400 leading-relaxed" { "Stunning brand identity designs, marketing assets, and corporate layouts curated by our professional design specialists." }
                            }
                        }
                    }
                }

                // --- EXPERIENCE ---
                section id="experience" class="py-24 border-t border-slate-900 bg-slate-900/20" {
                    div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8" {
                        div class="text-center mb-16" {
                            h2 class="text-3xl font-bold text-white mb-4" { "Track Record & Experience" }
                            p class="text-slate-400" { "A timeline of providing values and technical outputs over the years." }
                        }
                        
                        // Timeline Tree
                        div class="relative border-l border-slate-800 ml-4 md:ml-32 space-y-12" {
                            // Item 1
                            div class="relative pl-8" {
                                div class="absolute -left-3 top-1.5 w-6 h-6 rounded-full bg-slate-950 border-4 border-cyan-500 flex items-center justify-center" {}
                                div class="absolute -left-36 top-1 hidden md:block text-right w-28" {
                                    span class="text-sm font-bold text-cyan-400" { "2019 - Present" }
                                }
                                h3 class="text-xl font-bold text-white" { "CEO & Founder" }
                                p class="text-sm text-slate-400 font-medium" { "Adsea.id" }
                                p class="text-slate-400 text-sm mt-2 leading-relaxed" {
                                    "Directing end-to-end management, technological solutions architecture, and scaling digital marketing blueprints for micro-to-enterprise level businesses."
                                }
                            }
                            // Item 2
                            div class="relative pl-8" {
                                div class="absolute -left-3 top-1.5 w-6 h-6 rounded-full bg-slate-950 border-4 border-blue-500 flex items-center justify-center" {}
                                div class="absolute -left-36 top-1 hidden md:block text-right w-28" {
                                    span class="text-sm font-bold text-blue-400" { "2018 - Present" }
                                }
                                h3 class="text-xl font-bold text-white" { "Web & Video Production Lead" }
                                p class="text-sm text-slate-400 font-medium" { "Digital Consultancy" }
                                p class="text-slate-400 text-sm mt-2 leading-relaxed" {
                                    "Deploying robust business web channels (Laravel/Express) and high-quality video resources built to organically reach massive audiences across YouTube and broader social networks."
                                }
                            }
                        }
                    }
                }

                // --- FOOTER ---
                footer class="bg-slate-950 border-t border-slate-900 py-8 text-center text-xs text-slate-500" {
                    div class="max-w-6xl mx-auto px-4 space-y-3" {
                        div class="flex justify-center space-x-6 text-sm mb-2" {
                            a href="https://www.instagram.com/ariful.amar/" target="_blank" class="text-slate-400 hover:text-cyan-400 transition" { i class="fab fa-instagram text-lg" {} }
                            a href="mailto:Ariful.amar@gmail.com" class="text-slate-400 hover:text-cyan-400 transition" { i class="fas fa-envelope text-lg" {} }
                        }
                        p { "© 2026 Ariful Amar. All rights reserved." }
                        p class="font-mono text-[10px]" { "Compiled with 🦀 Rust — Blazing Fast, Pure Binary Web Server" }
                    }
                }
            }
        }
    };

    Html(markup.into_string())
}