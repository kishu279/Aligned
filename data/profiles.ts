export interface Prompt {
  question: string;
  answer: string;
}

export interface Profile {
  id: string;
  name: string;
  age: number;
  pronouns?: string;
  gender?: string;
  sexuality?: string;
  height?: string;
  location?: string;
  job?: string;
  ethnicity?: string;
  politics?: string;
  religion?: string;
  relationshipType?: string;
  datingIntention?: string;
  drinks?: string;
  smokes?: string;
  images: any[];
  prompts: Prompt[];
}

// Profile images
const images = {
  anaDeArmas: require("@/assets/images/ana-de-armas.jpg"),
  scarlett: require("@/assets/images/scarlett-jhonson.jpg"),
  elizabethOlsen: require("@/assets/images/elizabeth-olsen.jpg"),
  galGadot: require("@/assets/images/gal-gadot.jpg"),
  sadieSink: require("@/assets/images/sadie-sink.jpg"),
  dishaPatani: require("@/assets/images/disha-patani.jpg"),
  chrisEvans: require("@/assets/images/chris-evans.jpg"),
  chrisHemsworth: require("@/assets/images/chris-hemsworth.jpg"),
  henryCavil: require("@/assets/images/henry-cavil.jpg"),
  hrithikRoshan: require("@/assets/images/hrithik-roshan.jpg"),
  robertDowneyJr: require("@/assets/images/robert-downey-jr.jpg"),
  alexCosta: require("@/assets/images/alex-costa.jpg"),
};

export const profiles: Profile[] = [
  {
    id: "1",
    name: "Ana",
    age: 35,
    pronouns: "she/her",
    gender: "Woman",
    sexuality: "Straight",
    height: "5'6\"",
    location: "Los Angeles",
    job: "Actress",
    ethnicity: "Latina",
    politics: "Moderate",
    relationshipType: "Monogamy",
    datingIntention: "Long-term relationship",
    drinks: "Sometimes",
    smokes: "No",
    images: [images.anaDeArmas, images.anaDeArmas, images.anaDeArmas],
    prompts: [
      {
        question: "The way to win me over is",
        answer: "Good conversation and even better food",
      },
      {
        question: "Unusual skills",
        answer: "I can speak 3 languages fluently",
      },
      {
        question: "I'm looking for",
        answer: "Someone genuine who loves adventures",
      },
    ],
  },
  {
    id: "2",
    name: "Scarlett",
    age: 38,
    pronouns: "she/her",
    gender: "Woman",
    sexuality: "Straight",
    height: "5'3\"",
    location: "New York",
    job: "Actress",
    ethnicity: "White",
    politics: "Liberal",
    relationshipType: "Monogamy",
    datingIntention: "Long-term relationship",
    drinks: "Socially",
    smokes: "No",
    images: [images.scarlett, images.scarlett, images.scarlett],
    prompts: [
      {
        question: "The way to win me over is",
        answer: "Making me laugh uncontrollably",
      },
      {
        question: "My simple pleasures",
        answer: "Coffee, good books, and rainy days",
      },
      {
        question: "Dating me is like",
        answer: "A rollercoaster you never want to get off",
      },
    ],
  },
  {
    id: "3",
    name: "Elizabeth",
    age: 34,
    pronouns: "she/her",
    gender: "Woman",
    sexuality: "Straight",
    height: "5'4\"",
    location: "Los Angeles",
    job: "Actress",
    ethnicity: "White",
    politics: "Liberal",
    relationshipType: "Monogamy",
    datingIntention: "Long-term relationship",
    drinks: "Sometimes",
    smokes: "No",
    images: [images.elizabethOlsen, images.elizabethOlsen, images.elizabethOlsen],
    prompts: [
      {
        question: "Unusual skills",
        answer: "I can cook a 5-course meal in an hour",
      },
      {
        question: "The way to win me over is",
        answer: "Being passionate about something",
      },
      {
        question: "I'm looking for",
        answer: "A best friend I'm also attracted to",
      },
    ],
  },
  {
    id: "4",
    name: "Gal",
    age: 38,
    pronouns: "she/her",
    gender: "Woman",
    sexuality: "Straight",
    height: "5'10\"",
    location: "Tel Aviv",
    job: "Actress",
    ethnicity: "Middle Eastern",
    politics: "Moderate",
    relationshipType: "Monogamy",
    datingIntention: "Long-term relationship",
    drinks: "Sometimes",
    smokes: "No",
    images: [images.galGadot, images.galGadot, images.galGadot],
    prompts: [
      {
        question: "My simple pleasures",
        answer: "Working out at sunrise",
      },
      {
        question: "I'm convinced that",
        answer: "Love can conquer anything",
      },
      {
        question: "The way to win me over is",
        answer: "Being kind to everyone you meet",
      },
    ],
  },
  {
    id: "5",
    name: "Sadie",
    age: 22,
    pronouns: "she/her",
    gender: "Woman",
    sexuality: "Bisexual",
    height: "5'0\"",
    location: "New York",
    job: "Actress",
    ethnicity: "White",
    politics: "Liberal",
    relationshipType: "Figuring out my dating goals",
    datingIntention: "Figuring out my dating goals",
    drinks: "Sometimes",
    smokes: "No",
    images: [images.sadieSink, images.sadieSink, images.sadieSink],
    prompts: [
      {
        question: "Unusual skills",
        answer: "surviving on iced coffee",
      },
      {
        question: "I'm looking for",
        answer: "Someone who gets my weird humor",
      },
      {
        question: "The way to win me over is",
        answer: "Taking me to a concert",
      },
    ],
  },
  {
    id: "6",
    name: "Disha",
    age: 31,
    pronouns: "she/her",
    gender: "Woman",
    sexuality: "Straight",
    height: "5'7\"",
    location: "Mumbai",
    job: "Actress",
    ethnicity: "South Asian",
    politics: "Moderate",
    relationshipType: "Monogamy",
    datingIntention: "Long-term relationship",
    drinks: "No",
    smokes: "No",
    images: [images.dishaPatani, images.dishaPatani, images.dishaPatani],
    prompts: [
      {
        question: "My simple pleasures",
        answer: "Dancing in the rain",
      },
      {
        question: "The way to win me over is",
        answer: "Being fit and having a good sense of humor",
      },
      {
        question: "I'm convinced that",
        answer: "Good vibes attract good people",
      },
    ],
  },
  {
    id: "7",
    name: "Chris E",
    age: 42,
    pronouns: "he/him",
    gender: "Man",
    sexuality: "Straight",
    height: "6'0\"",
    location: "Boston",
    job: "Actor",
    ethnicity: "White",
    politics: "Liberal",
    relationshipType: "Monogamy",
    datingIntention: "Long-term relationship",
    drinks: "Socially",
    smokes: "No",
    images: [images.chrisEvans, images.chrisEvans, images.chrisEvans],
    prompts: [
      {
        question: "The way to win me over is",
        answer: "Love my dog as much as I do",
      },
      {
        question: "I'm looking for",
        answer: "Someone who appreciates a good pun",
      },
      {
        question: "My simple pleasures",
        answer: "Morning runs with my dog",
      },
    ],
  },
  {
    id: "8",
    name: "Chris H",
    age: 40,
    pronouns: "he/him",
    gender: "Man",
    sexuality: "Straight",
    height: "6'3\"",
    location: "Sydney",
    job: "Actor",
    ethnicity: "White",
    politics: "Moderate",
    relationshipType: "Monogamy",
    datingIntention: "Long-term relationship",
    drinks: "Sometimes",
    smokes: "No",
    images: [images.chrisHemsworth, images.chrisHemsworth, images.chrisHemsworth],
    prompts: [
      {
        question: "Unusual skills",
        answer: "I can surf any wave",
      },
      {
        question: "The way to win me over is",
        answer: "Loving the outdoors as much as me",
      },
      {
        question: "I'm convinced that",
        answer: "Australia has the best beaches",
      },
    ],
  },
  {
    id: "9",
    name: "Henry",
    age: 40,
    pronouns: "he/him",
    gender: "Man",
    sexuality: "Straight",
    height: "6'1\"",
    location: "London",
    job: "Actor",
    ethnicity: "White",
    politics: "Moderate",
    relationshipType: "Monogamy",
    datingIntention: "Long-term relationship",
    drinks: "Sometimes",
    smokes: "No",
    images: [images.henryCavil, images.henryCavil, images.henryCavil],
    prompts: [
      {
        question: "Unusual skills",
        answer: "Building custom gaming PCs",
      },
      {
        question: "The way to win me over is",
        answer: "Being my player 2",
      },
      {
        question: "I'm looking for",
        answer: "Someone who appreciates fantasy and sci-fi",
      },
    ],
  },
  {
    id: "10",
    name: "Hrithik",
    age: 50,
    pronouns: "he/him",
    gender: "Man",
    sexuality: "Straight",
    height: "5'11\"",
    location: "Mumbai",
    job: "Actor",
    ethnicity: "South Asian",
    politics: "Moderate",
    relationshipType: "Monogamy",
    datingIntention: "Long-term relationship",
    drinks: "No",
    smokes: "No",
    images: [images.hrithikRoshan, images.hrithikRoshan, images.hrithikRoshan],
    prompts: [
      {
        question: "My simple pleasures",
        answer: "Dancing like nobody's watching",
      },
      {
        question: "The way to win me over is",
        answer: "Having rhythm and soul",
      },
      {
        question: "I'm convinced that",
        answer: "Dance is the ultimate expression",
      },
    ],
  },
  {
    id: "11",
    name: "Robert",
    age: 58,
    pronouns: "he/him",
    gender: "Man",
    sexuality: "Straight",
    height: "5'8\"",
    location: "Los Angeles",
    job: "Actor",
    ethnicity: "White",
    politics: "Liberal",
    relationshipType: "Monogamy",
    datingIntention: "Long-term relationship",
    drinks: "No",
    smokes: "No",
    images: [images.robertDowneyJr, images.robertDowneyJr, images.robertDowneyJr],
    prompts: [
      {
        question: "Unusual skills",
        answer: "I can quote any 80s movie",
      },
      {
        question: "The way to win me over is",
        answer: "Having a witty comeback for everything",
      },
      {
        question: "I'm looking for",
        answer: "Someone who keeps me grounded",
      },
    ],
  },
  {
    id: "12",
    name: "Alex",
    age: 32,
    pronouns: "he/him",
    gender: "Man",
    sexuality: "Straight",
    height: "5'10\"",
    location: "Miami",
    job: "Creator",
    ethnicity: "Latino",
    politics: "Moderate",
    relationshipType: "Monogamy",
    datingIntention: "Long-term relationship",
    drinks: "Socially",
    smokes: "No",
    images: [images.alexCosta, images.alexCosta, images.alexCosta],
    prompts: [
      {
        question: "My simple pleasures",
        answer: "A fresh haircut and good coffee",
      },
      {
        question: "The way to win me over is",
        answer: "Having style and substance",
      },
      {
        question: "I'm looking for",
        answer: "A partner in life and business",
      },
    ],
  },
];
