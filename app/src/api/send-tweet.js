import { useWorkspace } from "@/composables";
import { web3 } from "@project-serum/anchor";
import { Tweet } from "@/models";

export const sendTweet = async (topic, content) => {
  // get wallet and program from anchor workspace
  const { wallet, program } = useWorkspace();

  // 1. Generate a new keypair
  const tweet = web3.Keypair.generate();

  // 2. Send a send tweet instruction with the right data and accounts
  await program.value.rpc.sendTweet(topic, content, {
    accounts: {
      author: wallet.value.publicKey,
      tweet: tweet?.publicKey,
      systemProgram: web3.SystemProgram.programId,
    },
    signers: [tweet],
  });

  // 3. Fetch the newly created tweet account from the blockchain
  const tweetAccount = await program.value.account.tweet.fetch(tweet.publicKey);

  //4. Wrap the fetched account in a tweet model for our frontend to display it
  return new Tweet(tweet.publicKey, tweetAccount);
};
