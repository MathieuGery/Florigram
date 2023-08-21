'use client'
import { useEffect, useState } from 'react'
import NavBar from '../components/NavBar'

function HandleError(props: any) {
  if (props.error != '') {
    return <a className='animate-pulse [animation-iteration-count:4] ml-4 text-red-500 text-l font-bold'>{props.error}</a>
  }
}

function WordGuess({ word }: { word: string }) {
  return (
    <div className='flex items-center justify-start'>
      {word.split('').map((char, index) => (
        <a key={index} className='m-1 px-2 shadow-xl rounded-lg ring-1 ring-teal-500 focus:outline-none focus:ring focus:ring-teal-500'>{char}</a>
      ))}
    </div>)
}

export default function Home() {
  const [word, setWord] = useState('')
  const [error, setError] = useState('')
  const [letters, setLetters] = useState('......')
  const [words, setWords] = useState([])

  function handleOnSubmit(event: any): void {
    event.preventDefault();
    if (word.includes(' ')) {
      setWord('')
      setError('😵‍💫 Un seul mot!')
      return
    }
    console.log(word)
    checkWord()
    setWord('')
    setError('')
  }

  const checkWord = async () => {
    try {
      const res = await fetch(
        process.env.NEXT_PUBLIC_API_URL + `/valide`,
        {
          method: 'POST',
          body: JSON.stringify({'word': word, 'words': words}),
          headers: { 'Content-Type': 'application/json'}
        }
      );
      const data = await res.json();
      console.log(data)
      if (data.is_valide) setWords(data.words)
      else setError('❌ Mauvais Mot !')
    } catch (err) {
      console.log(err);
    }
  };

  const callAPI = async () => {
    try {
      const res = await fetch(
        process.env.NEXT_PUBLIC_API_URL + `/day`,
        {
          headers: { 'Content-Type': 'application/json'}
        }
      );
      const data = await res.json();
      console.log(data)
      setWords(data.words)
      setLetters(data.letters)
    } catch (err) {
      console.log(err);
    }
  };

  useEffect(() => {
    callAPI()
  }, [])

  return (
    <>
      <div className='bg-zinc-800/40'>
        <NavBar />
        <div className='flex-row'>
          <div className='mx-5 rounded-xl py-5 mt-5'>
            <div className='my-5 mx-10 grid grid-rows-4 grid-flow-col gap-4 items-center justify-center'>
              {words.map((word, index) => (
                <WordGuess key={index} word={word} />
              ))}
            </div>
          </div>
          <div className='ring-1 ring-teal-500 mx-20 rounded-xl flex justify-center'>
            {letters.split('').map((char, index) => (
              <a key={index} className='text-2xl bold font-extrabold mx-2'>{char}</a>
            ))}
          </div>
          <div className='mx-10 mt-2'>
            <form className='mt-10' onSubmit={handleOnSubmit}>
              <HandleError error={error} />
              <input
                type='word'
                name='word'
                value={word}
                placeholder='🔎 Mot'
                aria-label='Mot'
                id='word'
                required
                onChange={event => setWord(event.target.value)}
                className='block w-full rounded-md border-0 px-2 py-1.5 text-white shadow-sm ring-1 ring-teal-700 placeholder:text-gray-500 focus:ring-2 focus:ring-teal-950 sm:text-sm sm:leading-6 bg-zinc-700/50'
              />
            </form>
          </div>
        </div>
      </div>
    </>
  )
}
