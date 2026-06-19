import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({ component: App });

const features = [
  {
    icon: "📦",
    title: "Comprehensive Package Format Support",
    desc: "Effortlessly generate platform-specific distribution files including APK, AAB, IPA, DMG, EXE, MSIX, DEB, RPM and more.",
  },
  {
    icon: "📱",
    title: "Seamless Distribution Platform Integration",
    desc: "Publish directly to major app marketplaces including Google Play Store, Apple App Store, App Gallery, Firebase, Pgyer, fir.im, and more.",
  },
  {
    icon: "⚙️",
    title: "Flexible Configuration Options",
    desc: "Customize your packaging and publishing process through intuitive yet powerful YAML configuration, supporting multiple environments and flavors.",
  },
  {
    icon: "🔄",
    title: "CI/CD Integration",
    desc: "Perfect integration with GitHub Actions, GitLab CI, and more. Automate your entire build and release pipeline with ease.",
  },
];

const platforms = [
  {
    label: "APK",
    color: "rgba(79,184,178,0.15)",
    border: "rgba(79,184,178,0.35)",
  },
  {
    label: "AAB",
    color: "rgba(79,184,178,0.15)",
    border: "rgba(79,184,178,0.35)",
  },
  {
    label: "IPA",
    color: "rgba(47,106,74,0.12)",
    border: "rgba(47,106,74,0.3)",
  },
  {
    label: "DMG",
    color: "rgba(47,106,74,0.12)",
    border: "rgba(47,106,74,0.3)",
  },
  {
    label: "EXE",
    color: "rgba(79,184,178,0.15)",
    border: "rgba(79,184,178,0.35)",
  },
  {
    label: "MSIX",
    color: "rgba(79,184,178,0.15)",
    border: "rgba(79,184,178,0.35)",
  },
  {
    label: "DEB",
    color: "rgba(47,106,74,0.12)",
    border: "rgba(47,106,74,0.3)",
  },
  {
    label: "RPM",
    color: "rgba(47,106,74,0.12)",
    border: "rgba(47,106,74,0.3)",
  },
  {
    label: "AppImage",
    color: "rgba(79,184,178,0.15)",
    border: "rgba(79,184,178,0.35)",
  },
  {
    label: "PKG",
    color: "rgba(47,106,74,0.12)",
    border: "rgba(47,106,74,0.3)",
  },
  {
    label: "HAP",
    color: "rgba(79,184,178,0.15)",
    border: "rgba(79,184,178,0.35)",
  },
  {
    label: "ZIP",
    color: "rgba(47,106,74,0.12)",
    border: "rgba(47,106,74,0.3)",
  },
];

const publishers = [
  "Google Play",
  "App Store",
  "App Gallery",
  "Firebase",
  "AppCenter",
  "Pgyer",
  "fir.im",
  "GitHub",
  "Vercel",
  "Qiniu",
];

function App() {
  return (
    <main className="page-wrap px-4 pb-16 pt-14">
      {/* Hero */}
      <section className="island-shell rise-in relative overflow-hidden rounded-[2rem] px-6 py-14 sm:px-12 sm:py-20">
        <div className="pointer-events-none absolute -left-20 -top-24 h-72 w-72 rounded-full bg-[radial-gradient(circle,rgba(79,184,178,0.32),transparent_66%)]" />
        <div className="pointer-events-none absolute -bottom-20 -right-20 h-72 w-72 rounded-full bg-[radial-gradient(circle,rgba(47,106,74,0.18),transparent_66%)]" />

        <p className="island-kicker mb-4">Flutter App Distribution</p>
        <h1 className="display-title mb-5 max-w-3xl text-4xl leading-[1.05] font-bold tracking-tight text-[var(--sea-ink)] sm:text-6xl">
          Make your app distribution with ease.
        </h1>
        <p className="mb-8 max-w-2xl text-base text-[var(--sea-ink-soft)] sm:text-lg">
          Build, package, and publish with unparalleled efficiency. Fastforge is
          an all-in-one Flutter application packaging and distribution tool — a
          one-stop solution for all your distribution needs.
        </p>

        <div className="flex flex-wrap gap-3">
          <a
            href="https://fastforge.dev/getting-started"
            className="rounded-full bg-[var(--lagoon-deep)] px-6 py-2.5 text-sm font-semibold text-white no-underline shadow-[0_4px_16px_rgba(50,143,151,0.35)] transition hover:-translate-y-0.5 hover:bg-[var(--lagoon)]"
          >
            Get Started
          </a>
          <a
            href="https://github.com/fastforgedev/fastforge"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-2 rounded-full border border-[rgba(23,58,64,0.2)] bg-white/50 px-6 py-2.5 text-sm font-semibold text-[var(--sea-ink)] no-underline transition hover:-translate-y-0.5 hover:border-[rgba(23,58,64,0.35)]"
          >
            <svg
              viewBox="0 0 16 16"
              aria-hidden="true"
              width="16"
              height="16"
              fill="currentColor"
            >
              <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z" />
            </svg>
            GitHub
          </a>
        </div>
      </section>

      {/* Quick Install */}
      <section
        className="island-shell rise-in mt-6 rounded-2xl px-6 py-5"
        style={{ animationDelay: "60ms" }}
      >
        <p className="island-kicker mb-2">Installation</p>
        <p className="mb-3 text-sm text-[var(--sea-ink-soft)]">
          Install Fastforge globally with a single command:
        </p>
        <div className="flex items-center gap-3 overflow-x-auto rounded-xl border border-[var(--line)] bg-[#1d2e45] px-5 py-3">
          <span className="select-none text-[rgba(141,229,219,0.5)]">$</span>
          <code className="border-0 bg-transparent p-0 text-sm text-[#e8efff]">
            dart pub global activate fastforge
          </code>
        </div>
      </section>

      {/* Features */}
      <section className="mt-6 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        {features.map(({ icon, title, desc }, index) => (
          <article
            key={title}
            className="island-shell feature-card rise-in rounded-2xl p-5"
            style={{ animationDelay: `${index * 80 + 120}ms` }}
          >
            <div className="mb-3 text-2xl">{icon}</div>
            <h2 className="mb-2 text-base font-semibold text-[var(--sea-ink)]">
              {title}
            </h2>
            <p className="m-0 text-sm text-[var(--sea-ink-soft)]">{desc}</p>
          </article>
        ))}
      </section>

      {/* Package Formats */}
      <section
        className="island-shell rise-in mt-6 rounded-2xl px-6 py-6"
        style={{ animationDelay: "460ms" }}
      >
        <p className="island-kicker mb-2">Package Formats</p>
        <p className="mb-4 text-sm text-[var(--sea-ink-soft)]">
          Generate distribution files for every major platform.
        </p>
        <div className="flex flex-wrap gap-2">
          {platforms.map(({ label, color, border }) => (
            <span
              key={label}
              className="rounded-full px-3 py-1 text-xs font-semibold text-[var(--sea-ink)]"
              style={{
                background: color,
                border: `1px solid ${border}`,
              }}
            >
              {label}
            </span>
          ))}
        </div>
      </section>

      {/* Publishers */}
      <section
        className="island-shell rise-in mt-6 rounded-2xl px-6 py-6"
        style={{ animationDelay: "520ms" }}
      >
        <p className="island-kicker mb-2">Distribution Platforms</p>
        <p className="mb-4 text-sm text-[var(--sea-ink-soft)]">
          Publish directly to the platforms your users rely on.
        </p>
        <div className="flex flex-wrap gap-2">
          {publishers.map((name) => (
            <span
              key={name}
              className="rounded-full border border-[rgba(50,143,151,0.3)] bg-[rgba(79,184,178,0.1)] px-3 py-1 text-xs font-semibold text-[var(--lagoon-deep)]"
            >
              {name}
            </span>
          ))}
        </div>
      </section>

      {/* Quick Start */}
      <section
        className="island-shell rise-in mt-6 rounded-2xl px-6 py-6"
        style={{ animationDelay: "580ms" }}
      >
        <p className="island-kicker mb-2">Quick Start</p>
        <p className="mb-4 text-sm text-[var(--sea-ink-soft)]">
          Add a <code>distribute_options.yaml</code> to your project root and
          define your release jobs:
        </p>
        <div className="overflow-x-auto rounded-xl border border-[var(--line)] bg-[#1d2e45] px-5 py-4 text-[#e8efff]">
          <pre className="m-0 text-xs leading-relaxed sm:text-sm">
            <code>{`variables:
  PGYER_API_KEY: 'your api key'
output: dist/
releases:
  - name: dev
    jobs:
      - name: release-dev-android
        package:
          platform: android
          target: apk
          build_args:
            flavor: dev
            dart-define:
              APP_ENV: dev
        publish_to: pgyer
      - name: release-dev-ios
        package:
          platform: ios
          target: ipa
          build_args:
            flavor: dev
            export-options-plist: ios/dev_ExportOptions.plist
            dart-define:
              APP_ENV: dev
        publish_to: pgyer`}</code>
          </pre>
        </div>
        <p className="mt-4 mb-0 text-sm text-[var(--sea-ink-soft)]">
          Then run: <code>fastforge release --name dev</code>
        </p>
      </section>

      {/* CTA */}
      <section
        className="island-shell rise-in relative mt-6 overflow-hidden rounded-2xl px-6 py-10 text-center sm:px-10"
        style={{ animationDelay: "640ms" }}
      >
        <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_50%_0%,rgba(79,184,178,0.18),transparent_60%)]" />
        <h2 className="display-title relative mb-3 text-2xl font-bold text-[var(--sea-ink)] sm:text-3xl">
          Ready to simplify your Flutter releases?
        </h2>
        <p className="relative mb-6 text-sm text-[var(--sea-ink-soft)] sm:text-base">
          Join developers who ship faster with Fastforge.
        </p>
        <div className="relative flex flex-wrap justify-center gap-3">
          <a
            href="https://fastforge.dev/getting-started"
            className="rounded-full bg-[var(--lagoon-deep)] px-6 py-2.5 text-sm font-semibold text-white no-underline shadow-[0_4px_16px_rgba(50,143,151,0.35)] transition hover:-translate-y-0.5 hover:bg-[var(--lagoon)]"
          >
            Read the Docs
          </a>
          <a
            href="https://github.com/fastforgedev/fastforge"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-2 rounded-full border border-[rgba(23,58,64,0.2)] bg-white/50 px-6 py-2.5 text-sm font-semibold text-[var(--sea-ink)] no-underline transition hover:-translate-y-0.5 hover:border-[rgba(23,58,64,0.35)]"
          >
            Star on GitHub
          </a>
        </div>
      </section>
    </main>
  );
}
