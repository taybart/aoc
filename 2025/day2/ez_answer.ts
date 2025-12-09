async function main() {
  Bun.file('example.txt').text().then((text: string) => {
    const ids = text.split(',').map((range: string) => {
      const [start, end] = range.split('-').map((n) => parseInt(n))
      return { start, end }
    })
    let total = 0
    for (let range of ids) {
      for (let i = range.start; i <= range.end; ++i) {
        // split i in half and compare halfs
        const arr = String(i).split("")
        const badID = arr.slice(0, arr.length / 2).join("") === arr.slice(arr.length / 2).join("")
        if (badID) {
          total += i
        }
      }
    }
    console.log(total)
  })
}
main()
