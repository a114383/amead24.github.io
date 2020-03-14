Vue.component('product-review', {
    template: `
    <form class="review-form" @submit.prevent="onSubmit">
    <p>
      <label for="name">Name:</label>
      <input id="name" v-model="name" placeholder="name">
    </p>
    
    <p>
      <label for="review">Review:</label>      
      <textarea id="review" v-model="review"></textarea>
    </p>
    
    <p>
      <label for="rating">Rating:</label>
      <select id="rating" v-model.number="rating">
        <option>5</option>
        <option>4</option>
        <option>3</option>
        <option>2</option>
        <option>1</option>
      </select>
    </p>
        
    <p>
      <input type="submit" value="Submit">  
    </p>    
  
    </form>
    `,
    data() {
        return {
            name: null,
            rating: null,
            review: null,
        }
    },
    methods: {
        onSubmit() {
            let productReview = {
                name: this.name,
                review: this.review,
                rating: this.rating,
            }

            this.$emit('review-submitted', productReview)

            this.name = null
            this.review = null
            this.rating = null
        }
    }
})


Vue.component('product-details', {
    props: {
        details: {
            type: Array,
            require: true,
        },
    },
    template: `
    <div>
        <ul>
            <li v-for="detail in details">{{ detail }}</li>
        </ul>
    </div>
    `
})


Vue.component('product', {
    props: {
        premium: {
            type: Boolean,
            required: true,
        },
    },
    template: `
    <div class="product">
        <div class="product-image">
            <!-- v-bind shortcut is just :<attributes> -->
            <a :href="productHref"><img :src="image"></a>
        </div>

        <div class="product-info">
            <!-- 
                avoid interpolation inside attributes - id={{ ?? }} 
                and instead stick to v-bind or color shorthand
            -->
            <h1 :id="product">{{ title }}</h1>
            <h3>{{ description }}</h3>


            <!-- vs. v-show , more performative that adding & removing divs -->
            <p v-if="soldOut"> Out of Stock! </p>
            <p v-else-if="inStock && onSale">In stock now and on sale!</p>
            <p v-else-if="inStock">In stock now</p>
            <p v-else-if="lowQuantity && onSale"> Almost sold out and it's on sale </p>
            <p v-else-if="lowQuantity">Almost sold out!</p>
            <p v-else>ERROR!</p>

            <p> Shipping: {{ shipping }}</p>


            <product-details :details="details"></product-details>


            <!-- both of these options go for :style or :class
                :style with conditionals - :style=["attr" ? "isEnabled" : "isNotEnabled"] 
                :style with objects - :style="someObject" => data.someObject = {"attr": "value", ...}
                :style with arrays - :style=["attr0", ..., "attrN"]
            -->
            <!-- :key attribute is just a recommendation? -->
            <div v-for="(variant, index) in variants"
                    :key="variant.id"
                    class="color-box"
                    :style="{ backgroundColor: variant.color }"
                    @mouseover="updateProduct(index)"
            >
            </div>


            <div>
                <!-- @ symbol shorthand for v-on -->
                <button @click="addToCart"
                        :disabled="soldOut">
                Add to Cart
                </button>

                <button @click="removeFromCart">Remove From Cart</button>
            </div>

        </div>

        <div id="product-reviews">
            <product-review @review-submitted="addReview"></product-review>
        </div>
    </div>
    `,
    data() {
        return {
            brand: 'colored',
            product: 'Socks',
            description: 'Those fuzy things on your feet',
            // setting default - refactored for variantIndex
            // image: './assets/vmSocks-green-onWhite.jpg',
            variantIndex: 0,
            details: ["warm", "fuzzy", "fashionable"],
            variants: [
                {
                    id: 1000,
                    color: "green",
                    image: "./assets/vmSocks-green-onWhite.jpg",
                    quantity: 20,
                    onSale: true,
                },
                {
                    id: 1001,
                    color: "blue",
                    image: "./assets/vmSocks-blue-onWhite.jpg",
                    quantity: 2,
                    onSale: false,
                },
            ],
            reviews: []
        }
    },
    methods: {
        addToCart: function () {
            this.$emit('add-to-cart', this.variants[this.variantIndex].id)
        },
        removeFromCart() {
            this.$emit('remove-from-cart', this.variants[this.variantIndex].id)
        },
        // Can be anonymous or ES6 short hand
        updateProduct(index) {
            console.log(`swapping ${this.variantIndex} for ${index}.`)
            this.variantIndex = index
        },
        addReview(productReview) {
            this.reviews.push(productReview)
        }
    },
    computed: {
        // Note computed properties are chached until their data elements change
        // thus it's suggested these are used over methods
        title() { return this.brand + ' ' + this.product },
        productHref() { return '#' + this.product.toLocaleLowerCase() },

        image() { return this.variants[this.variantIndex].image },
        soldOut() { return this.variants[this.variantIndex].quantity == 0 },
        onSale() { return this.variants[this.variantIndex].onSale },
        inStock() { return this.variants[this.variantIndex].quantity > 10 },
        lowQuantity() { 
            return (
                this.variants[this.variantIndex].quantity > 0 &&
                this.variants[this.variantIndex].quantity <= 10
            )
        },

        shipping() { return (this.premium) ? "Free!" : "$2.99" }
    }
})


var app = new Vue({
    el: '#app',
    data: {
        premium: true,
        cart: [],
    },
    methods: {
        updateCart(id) { this.cart.push(id) },
        removeFromCart(id) { this.cart.pop(id) }
    }
})
