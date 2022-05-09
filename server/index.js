const { analyze } = require("./pkg")
const fastify = require('fastify')({ logger: true })

fastify.post('/', async (request, reply) =>  {
  return analyze( request.body ) 
})

const start = async () => {
  try {
    await fastify.listen(3000)
  } catch (err) {
    fastify.log.error(err)
    process.exit(1)
  }
}

start()
