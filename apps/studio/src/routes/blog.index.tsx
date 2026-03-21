import { Link, createFileRoute } from '@tanstack/react-router'
import { allBlogs } from 'content-collections'
import { SITE_DESCRIPTION, SITE_TITLE, SITE_URL } from '#/lib/site'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'

const canonical = `${SITE_URL}/blog`
const pageTitle = `Blog | ${SITE_TITLE}`

export const Route = createFileRoute('/blog/')({
  head: () => ({
    links: [{ rel: 'canonical', href: canonical }],
    meta: [
      { title: pageTitle },
      { name: 'description', content: SITE_DESCRIPTION },
      { property: 'og:image', content: `${SITE_URL}/images/lagoon-1.svg` },
    ],
  }),
  component: BlogIndex,
})

function BlogIndex() {
  const postsByDate = Array.from(
    new Map(
      [...allBlogs]
        .sort(
          (a, b) =>
            new Date(b.pubDate).valueOf() - new Date(a.pubDate).valueOf(),
        )
        .map((post) => [post.slug, post]),
    ).values(),
  )

  const featured = postsByDate[0]
  const posts = postsByDate.slice(1)
  return (
    <main className="mx-auto w-full max-w-6xl px-4 pb-8 pt-14">
      <section className="mb-4">
        <p className="mb-2 text-sm font-medium text-muted-foreground">Latest Dispatches</p>
        <h1 className="m-0 text-4xl font-semibold tracking-tight sm:text-5xl">
          Blog
        </h1>
      </section>

      <section className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        <Card className="lg:col-span-2">
          {featured.heroImage ? (
            <img
              src={featured.heroImage}
              alt=""
              className="h-44 w-full rounded-t-xl object-cover xl:h-60"
            />
          ) : null}
          <CardHeader className="p-5 sm:p-6">
            <CardTitle className="text-2xl">
              <Link to="/blog/$slug" params={{ slug: featured.slug }} className="hover:underline">
                {featured.title}
              </Link>
            </CardTitle>
          </CardHeader>
          <CardContent className="p-5 pt-0 sm:p-6 sm:pt-0">
            <p className="mb-3 text-base text-muted-foreground">{featured.description}</p>
            <p className="m-0 text-xs text-muted-foreground">
              {new Date(featured.pubDate).toLocaleDateString()}
            </p>
          </CardContent>
        </Card>

        {posts.map((post, index) => (
          <Card key={post.slug} className="sm:last:col-span-2 lg:last:col-span-1">
            {post.heroImage ? (
              <img
                src={post.heroImage}
                alt=""
                className="h-44 w-full rounded-t-xl object-cover"
              />
            ) : null}
            <CardHeader className="p-5">
              <CardTitle className="text-2xl">
                <Link to="/blog/$slug" params={{ slug: post.slug }} className="hover:underline">
                  {post.title}
                </Link>
              </CardTitle>
            </CardHeader>
            <CardContent className="p-5 pt-0">
              <p className="mb-2 text-sm text-muted-foreground">{post.description}</p>
              <p className="m-0 text-xs text-muted-foreground">
                {new Date(post.pubDate).toLocaleDateString()}
              </p>
            </CardContent>
          </Card>
        ))}
      </section>
    </main>
  )
}
