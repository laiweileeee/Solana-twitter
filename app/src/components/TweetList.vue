<script setup>
import { computed, toRefs } from "vue";
import TweetCard from "@/components/TweetCard";

const props = defineProps({
  tweets: Array,
  loading: Boolean,
});

const { tweets, loading } = toRefs(props);
const orderedTweets = computed(() => {
  return tweets.value.slice().sort((a, b) => b.timestamp - a.timestamp);
});

//Listen to 'delete' event
const emit = defineEmits(["update:tweets"]);

const onDelete = (deletedTweet) => {
  const filteredTweets = tweets.value.filter(
    (tweet) => tweet.publicKey.toBase58() !== deletedTweet.publicKey.toBase58()
  );
  emit("update:tweets", filteredTweets);
};
</script>

<template>
  <div v-if="loading" class="p-8 text-gray-500 text-center">Loading...</div>
  <div v-else class="divide-y">
    <tweet-card
      @delete="onDelete"
      v-for="tweet in orderedTweets"
      :key="tweet.key"
      :tweet="tweet"
    ></tweet-card>
  </div>
</template>
